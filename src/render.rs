extern crate image;
extern crate rand;
extern crate rayon;

use image::{ImageBuffer, Rgb, RgbImage};
use rand::{Rng, ThreadRng};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use std::f64::consts::PI;
use std::cmp::min;

use element::Color;
use shapes::*;
use calculate::distance;

struct EntityIntersection {
    point: (f64, f64),
    normal: (f64, f64),
    emissive: Color,
    reflectivity: f64,
    eta: f64,
    absorption: Color,
}

pub struct Entity {
    pub shape: Box<Shape + Sync>,
    pub emissive: Color,
    pub reflectivity: f64,
    pub eta: f64,
    pub absorption: Color,
}

impl Entity {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Option<EntityIntersection> {
        self.shape.intersect(p, d).map(|intersection| EntityIntersection {
            point: intersection.point,
            normal: intersection.normal,
            emissive: self.emissive.clone(),
            reflectivity: self.reflectivity,
            eta: self.eta,
            absorption: self.absorption,
        })
    }
}

pub struct Scene {
    pub entities: Vec<Entity>,
}


impl Scene {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Option<EntityIntersection> {
        let mut res: Option<EntityIntersection> = None;
        for e in &self.entities {
            if let Some(intersection) = e.intersect(p, d) {
                res = match res {
                    Some(r) => {
                        if distance(p, r.point) > distance(p, intersection.point) {
                            Some(intersection)
                        } else {
                            Some(r)
                        }
                    }
                    None => Some(intersection),
                }
            }
        }
        res
    }
}

fn reflect(ix: f64, iy: f64, nx: f64, ny: f64) -> (f64, f64) {
    let dot2 = (ix * nx + iy * ny) * 2.0;
    (ix - dot2 * nx, iy - dot2 * ny)
}

fn refract(ix: f64, iy: f64, nx: f64, ny: f64, eta: f64) -> Option<(f64, f64)> {
    let dot = ix * nx + iy * ny;
    let k = 1.0 - eta * eta * (1.0 - dot * dot);
    if k < 0.0 {
        return None; // all reflection
    }
    let a = eta * dot + k.sqrt();
    Some((eta * ix - a * nx, eta * iy - a * ny))
}

#[allow(dead_code)]
fn fresnel(cosi: f64, cost: f64, etai: f64, etat: f64) -> f64 {
    let rs = (etat * cosi - etai * cost) / (etat * cosi + etai * cost);
    let rp = (etat * cost - etai * cosi) / (etat * cost + etai * cosi);
    (rs * rs + rp * rp) * 0.5
}

fn schlick(cosi: f64, cost: f64, etai: f64, etat: f64) -> f64 {
    let r0 = (etai - etat) / (etai + etat);
    let r0 = r0 * r0;
    let a = if etai < etat {
        1.0 - cosi
    } else {
        1.0 - cost
    };
    let aa = a * a;
    r0 + (1.0 - r0) * aa * aa * a
}

fn beer_lambert(a: Color, d: f64) -> Color {
    Color {
        r: (-a.r * d).exp(),
        g: (-a.g * d).exp(),
        b: (-a.b * d).exp(),
    }
}

fn trace(scene: &Scene, ox: f64, oy: f64, dx: f64, dy: f64, depth: u32) -> Color {
    if let Some(r) = scene.intersect((ox, oy), (dx, dy)) {
        let sign = if r.normal.0 * dx + r.normal.1 * dy < 0.0 {
            1.0
        } else {
            -1.0
        };
        let mut sum = r.emissive;
        if depth > 0 && (r.reflectivity > 0.0 || r.eta > 0.0) {
            let mut refl = r.reflectivity;
            let (x, y) = r.point;
            let nx = r.normal.0 * sign;
            let ny = r.normal.1 * sign;
            if r.eta > 0.0 {
                let eta = if sign < 0.0 {
                    r.eta
                } else {
                    1.0 / r.eta
                };
                match refract(dx, dy, nx, ny, eta) {
                    Some((rx, ry)) => {
                        let cosi = -(dx * nx + dy * ny);
                        let cost = -(rx * nx + ry * ny);
                        refl = if sign < 0.0 {
                            schlick(cosi, cost, r.eta, 1.0)
                        } else {
                            schlick(cosi, cost, 1.0, r.eta)
                        };
                        sum = sum + trace(scene, x, y, rx, ry, depth - 1) * (1.0 - refl)
                    }
                    None => {
                        refl = 1.0
                    }
                }
            }
            if refl > 0.0 {
                let (rx, ry) = reflect(dx, dy, nx, ny);
                sum = sum + trace(scene, x, y, rx, ry, depth - 1) * refl;
            }
        }
        if sign < 0.0 {
            sum = sum * beer_lambert(r.absorption, distance((ox, oy), r.point));
        }
        sum
    } else {
        Color::black()
    }
}

fn render_point(scene: &Scene, stratification: u32, max_depth: u32, rng: &mut ThreadRng, point: (f64, f64)) -> Color {
    let sum: Color = (0..stratification).map(|i| 2.0 * PI * (i as f64 + rng.gen_range(0.0, 1.0)) / stratification as f64)
        .collect::<Vec<f64>>()
        .par_iter()
        .map(|a| trace(scene, point.0, point.1, a.cos(), a.sin(), max_depth))
        .sum();
    sum * (1.0 / stratification as f64)
}

pub fn render(scene: &Scene, size: (u32, u32), stratification: u32, max_depth: u32) -> RgbImage {
    let mut img = ImageBuffer::from_pixel(size.0, size.1, Rgb([0u8, 0u8, 0u8]));
    let mut rng = rand::thread_rng();
    for x in 0..size.0 {
        for y in 0..size.1 {
            let xx = x as f64 / size.0 as f64;
            let yy = y as f64 / size.1 as f64;
            let color = render_point(&scene, stratification, max_depth, &mut rng, (xx, yy));
            let r = min((color.r * 255.0) as u32, 255) as u8;
            let g = min((color.g * 255.0) as u32, 255) as u8;
            let b = min((color.b * 255.0) as u32, 255) as u8;
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    img
}