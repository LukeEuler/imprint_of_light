use std::f64::consts::PI;

const EPSILON: f64 = 1e-6;
const WHOLE_ANGLE: f64 = 360.0;


#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub point: (f64, f64),
    pub normal: (f64, f64),
}

pub trait Shape {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Vec<Intersection>;
    fn is_inside(&self, p: (f64, f64)) -> bool;
}

#[allow(dead_code)]
pub struct Circle {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

impl Shape for Circle {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Vec<Intersection> {
        let a = d.0 * d.0 + d.1 * d.1;
        let ocx = p.0 - self.cx;
        let ocy = p.1 - self.cy;
        let b = 2.0 * (ocx * d.0 + ocy * d.1);
        let c = ocx * ocx + ocy * ocy - self.r * self.r;
        let delta = b * b - 4.0 * a * c;
        let mut result: Vec<Intersection> = Vec::new();
        if delta < 0.0 {
            result
        } else {
            let t1 = (-b - delta.sqrt()) / (2.0 * a);
            let t2 = (-b + delta.sqrt()) / (2.0 * a);
            if t1 > EPSILON {
                let x = p.0 + d.0 * t1;
                let y = p.1 + d.1 * t1;
                let nx = x - self.cx;
                let ny = y - self.cy;
                let len = (nx * nx + ny * ny).sqrt();
                result.push(Intersection {
                    point: (x, y),
                    normal: (nx / len, ny / len),
                });
            }
            if t2 > EPSILON {
                let x = p.0 + d.0 * t2;
                let y = p.1 + d.1 * t2;
                let nx = x - self.cx;
                let ny = y - self.cy;
                let len = (nx * nx + ny * ny).sqrt();
                result.push(Intersection {
                    point: (x, y),
                    normal: (nx / len, ny / len),
                });
            }
            result
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
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Vec<Intersection> {
        let mut result: Vec<Intersection> = Vec::new();
        let a = d.0 * self.nx + d.1 * self.ny;
        if a.abs() < EPSILON {
            result
        } else {
            let b = (self.px - p.0) * self.nx + (self.py - p.1) * self.ny;
            let t = b / a;
            if t > EPSILON {
                result.push(Intersection {
                    point: (p.0 + d.0 * t, p.1 + d.1 * t),
                    normal: (self.nx, self.ny),
                });
            }
            result
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

    pub fn rectangle(cx: f64, cy: f64, e: f64, sx: f64, sy: f64) -> Self {
        let mut elevation = e;
        while elevation < 0.0 {
            elevation += WHOLE_ANGLE;
        }
        while elevation >= WHOLE_ANGLE {
            elevation -= WHOLE_ANGLE
        }
        let theta = 2.0 * PI * elevation / WHOLE_ANGLE;
        Self::new([(sx, -sy), (-sx, -sy), (-sx, sy), (sx, sy)].iter()
            .map(|&(x, y)| (x * theta.cos() - y * theta.sin(), x * theta.sin() + y * theta.cos()))
            .map(|(x, y)| (x + cx, y + cy))
            .collect())
    }

    pub fn regular(cx: f64, cy: f64, r: f64, n: u32, e: f64) -> Self {
        let mut elevation = e;
        while elevation < 0.0 {
            elevation += WHOLE_ANGLE;
        }
        while elevation >= WHOLE_ANGLE {
            elevation -= WHOLE_ANGLE
        }
        Self::new((0..n).map(|i| i as f64 * 2.0 * PI / n as f64)
            .map(|theta| (theta + 2.0 * PI * elevation / WHOLE_ANGLE))
            .map(|theta| (r * theta.cos(), r * theta.sin()))
            .map(|(x, y)| (cx + x, cy - y))
            .collect())
    }
}

impl Shape for Polygon {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Vec<Intersection> {
        let mut result: Vec<Intersection> = Vec::new();
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
                        result.push(Intersection {
                            point: (p.0 + d.0 * t, p.1 + d.1 * t),
                            normal: (nx, ny),
                        });
                    }
                }
            }
        }
        result
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
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Vec<Intersection> {
        let mut result: Vec<Intersection> = Vec::new();
        for item in self.a.intersect(p, d) {
            if !self.b.is_inside(item.point) {
                result.push(item);
            }
        }
        for item in self.b.intersect(p, d) {
            if !self.a.is_inside(item.point) {
                result.push(item);
            }
        }
        result
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
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Vec<Intersection> {
        let mut result: Vec<Intersection> = Vec::new();
        for item in self.a.intersect(p, d) {
            if self.b.is_inside(item.point) {
                result.push(item);
            }
        }
        for item in self.b.intersect(p, d) {
            if self.a.is_inside(item.point) {
                result.push(item);
            }
        }
        result
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

#[allow(dead_code)]
pub struct ComplementShape {
    pub a: Box<Shape + Sync>,
}

impl Shape for ComplementShape {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Vec<Intersection> {
        let mut result: Vec<Intersection> = Vec::new();
        for item in self.a.intersect(p, d) {
            let mut opposite_item = item;
            opposite_item.normal.0 = -opposite_item.normal.0;
            opposite_item.normal.1 = -opposite_item.normal.1;
            result.push(opposite_item);
        }
        result
    }

    fn is_inside(&self, p: (f64, f64)) -> bool {
        !self.a.is_inside(p)
    }
}

#[allow(dead_code)]
impl ComplementShape {
    fn new(a: Box<Shape + Sync>) -> ComplementShape {
        ComplementShape {
            a
        }
    }
}