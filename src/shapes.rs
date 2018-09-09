use std::f64::consts::PI;
use calculate::distance;

const EPSILON: f64 = 1e-6;


#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub point: (f64, f64),
    pub normal: (f64, f64),
}

pub trait Shape {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Option<Intersection>;
    fn is_inside(&self, p: (f64, f64)) -> bool;
}

#[allow(dead_code)]
pub struct Circle {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

impl Shape for Circle {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Option<Intersection> {
        let a = d.0 * d.0 + d.1 * d.1;
        let ocx = p.0 - self.cx;
        let ocy = p.1 - self.cy;
        let b = 2.0 * (ocx * d.0 + ocy * d.1);
        let c = ocx * ocx + ocy * ocy - self.r * self.r;
        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            None
        } else {
            let t1 = (-b - delta.sqrt()) / (2.0 * a);
            let t2 = (-b + delta.sqrt()) / (2.0 * a);
            let t = if t1 > EPSILON {
                t1
            } else {
                t2
            };
            if t > EPSILON {
                let x = p.0 + d.0 * t;
                let y = p.1 + d.1 * t;
                let nx = x - self.cx;
                let ny = y - self.cy;
                let len = (nx * nx + ny * ny).sqrt();
                Some(Intersection {
                    point: (x, y),
                    normal: (nx / len, ny / len),
                })
            } else {
                None
            }
        }
    }

    fn is_inside(&self, p: (f64, f64)) -> bool {
        let x = p.0 - self.cx;
        let y = p.1 - self.cy;
        x * x + y * y < self.r * self.r
    }
}

#[allow(dead_code)]
pub struct Plane {
    pub px: f64,
    pub py: f64,
    pub nx: f64,
    pub ny: f64,
}

impl Shape for Plane {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Option<Intersection> {
        let a = d.0 * self.nx + d.1 * self.ny;
        if a.abs() < EPSILON {
            None
        } else {
            let b = (self.px - p.0) * self.nx + (self.py - p.1) * self.ny;
            let t = b / a;
            if t > EPSILON {
                Some(Intersection {
                    point: (p.0 + d.0 * t, p.1 + d.1 * t),
                    normal: (self.nx, self.ny),
                })
            } else {
                None
            }
        }
    }

    fn is_inside(&self, p: (f64, f64)) -> bool {
        (p.0 - self.px) * self.nx + (p.1 - self.py) * self.ny < 0.0
    }
}

pub struct Polygon {
    points: Vec<(f64, f64)>, // counterclockwise
}

#[allow(dead_code)]
impl Polygon {
    pub fn new(p: Vec<(f64, f64)>) -> Self {
        if p.len() > 1 {
            Self {
                points: p,
            }
        } else {
            panic!("Too few points!");
        }
    }

    pub fn rectangle(cx: f64, cy: f64, theta: f64, sx: f64, sy: f64) -> Self {
        Self::new([(sx, -sy), (-sx, -sy), (-sx, sy), (sx, sy)].iter()
            .map(|&(x, y)| (x * theta.cos() - y * theta.sin(), x * theta.sin() + y * theta.cos()))
            .map(|(x, y)| (x + cx, y + cy))
            .collect())
    }

    pub fn ngon(cx: f64, cy: f64, r: f64, n: u32) -> Self {
        Self::new((0..n).map(|i| i as f64 * 2.0 * PI / n as f64)
            .map(|theta| (r * theta.cos(), r * theta.sin()))
            .map(|(x, y)| (cx + x, cy - y))
            .collect())
    }
}

impl Shape for Polygon {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Option<Intersection> {
        let mut res: Option<Intersection> = None;
        for i in 0..self.points.len() {
            let a = self.points[i];
            let b = if i + 1 == self.points.len() {
                self.points[0]
            } else {
                self.points[i + 1]
            };
            let ax = a.0 - p.0;
            let ay = a.1 - p.1;
            let bx = b.0 - p.0;
            let by = b.1 - p.1;
            let product1 = ax * d.1 - d.0 * ay;
            let product2 = bx * d.1 - d.0 * by;
            if product1 * product2 < 0.0 {
                let nx = a.1 - b.1;
                let ny = b.0 - a.0;
                let len = (nx * nx + ny * ny).sqrt();
                let nx = nx / len;
                let ny = ny / len;
                let c1 = d.0 * nx + d.1 * ny;
                if c1.abs() > EPSILON {
                    let c2 = (a.0 - p.0) * nx + (a.1 - p.1) * ny;
                    let t = c2 / c1;
                    if t > EPSILON {
                        let intersect = Intersection {
                            point: (p.0 + d.0 * t, p.1 + d.1 * t),
                            normal: (nx, ny),
                        };
                        res = match res {
                            Some(i) => {
                                if distance(p, intersect.point) < distance(p, i.point) {
                                    Some(intersect)
                                } else {
                                    Some(i)
                                }
                            }
                            None => Some(intersect),
                        }
                    }
                }
            }
        }
        res
    }

    fn is_inside(&self, p: (f64, f64)) -> bool {
        for i in 0..self.points.len() {
            let a = self.points[i];
            let b = if i + 1 == self.points.len() {
                self.points[0]
            } else {
                self.points[i + 1]
            };
            let ax = b.0 - a.0;
            let ay = b.1 - a.1;
            let bx = p.0 - a.0;
            let by = p.1 - a.1;
            if ax * by - bx * ay >= 0.0 {
                return false;
            }
        }
        true
    }
}

#[allow(dead_code)]
pub struct UnionShape {
    pub a: Box<Shape + Sync>,
    pub b: Box<Shape + Sync>,
}

impl Shape for UnionShape {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Option<Intersection> {
        match (self.a.intersect(p, d), self.b.intersect(p, d)) {
            (Some(i1), Some(i2)) => {
                let d1 = distance(p, i1.point);
                let d2 = distance(p, i2.point);
                if d1 < d2 {
                    Some(i1)
                } else {
                    Some(i2)
                }
            }
            (None, r2) => r2,
            (r1, None) => r1,
        }
    }

    fn is_inside(&self, p: (f64, f64)) -> bool {
        self.a.is_inside(p) || self.b.is_inside(p)
    }
}

#[allow(dead_code)]
impl UnionShape {
    fn new(a: Box<Shape + Sync>, b: Box<Shape + Sync>) -> UnionShape {
        UnionShape {
            a,
            b,
        }
    }
}

#[allow(dead_code)]
pub struct IntersectShape {
    pub a: Box<Shape + Sync>,
    pub b: Box<Shape + Sync>,
}

impl Shape for IntersectShape {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Option<Intersection> {
        match (self.a.intersect(p, d), self.b.intersect(p, d)) {
            (Some(i1), Some(i2)) => {
                if self.a.is_inside(i2.point) && self.b.is_inside(i1.point) {
                    let d1 = distance(p, i1.point);
                    let d2 = distance(p, i2.point);
                    if d1 < d2 {
                        Some(i1)
                    } else {
                        Some(i2)
                    }
                } else if self.a.is_inside(i2.point) {
                    Some(i2)
                } else if self.b.is_inside(i1.point) {
                    Some(i1)
                } else {
                    None
                }
            }
            (None, Some(i2)) => {
                if self.a.is_inside(i2.point) {
                    Some(i2)
                } else {
                    None
                }
            }
            (Some(i1), None) => {
                if self.b.is_inside(i1.point) {
                    Some(i1)
                } else {
                    None
                }
            }
            (None, None) => None,
        }
    }

    fn is_inside(&self, p: (f64, f64)) -> bool {
        self.a.is_inside(p) && self.b.is_inside(p)
    }
}

#[allow(dead_code)]
impl IntersectShape {
    fn new(a: Box<Shape + Sync>, b: Box<Shape + Sync>) -> IntersectShape {
        IntersectShape {
            a,
            b,
        }
    }
}