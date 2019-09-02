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
pub struct DirectionalLight {
    pub d: f64,
    pub nx: f64,
    pub ny: f64,
}

impl Shape for DirectionalLight {
    fn intersect(&self, (px, py): (f64, f64), (dx, dy): (f64, f64)) -> Vec<Intersection> {
        let mut result: Vec<Intersection> = Vec::new();
        let c = dx * self.nx + dy * self.ny;
        if c < EPSILON {
            return result;
        }
        let a = (dx * dx + dy * dy).sqrt();
        let b = (self.nx * self.nx + self.ny * self.ny).sqrt();
        let t = (c / (a * b)).acos();
        if t.abs() < 0.09 {
            result.push(Intersection {
                point: (px + self.d * dx / a, py + self.d * dy / a),
                normal: (self.nx, self.ny),
            });
        }
        result
    }

    fn is_inside(&self, _: (f64, f64)) -> bool {
        false
    }
}

#[allow(dead_code)]
pub struct Circle {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

impl Shape for Circle {
    fn intersect(&self, (px, py): (f64, f64), (dx, dy): (f64, f64)) -> Vec<Intersection> {
        let a = dx * dx + dy * dy;
        let ocx = px - self.cx;
        let ocy = py - self.cy;
        let b = 2.0 * (ocx * dx + ocy * dy);
        let c = ocx * ocx + ocy * ocy - self.r * self.r;
        let delta = b * b - 4.0 * a * c;
        let mut result: Vec<Intersection> = Vec::new();
        if delta < 0.0 {
            result
        } else {
            let t1 = (-b - delta.sqrt()) / (2.0 * a);
            let t2 = (-b + delta.sqrt()) / (2.0 * a);
            if t1 > EPSILON {
                let x = px + dx * t1;
                let y = py + dy * t1;
                let nx = x - self.cx;
                let ny = y - self.cy;
                let len = (nx * nx + ny * ny).sqrt();
                result.push(Intersection {
                    point: (x, y),
                    normal: (nx / len, ny / len),
                });
            }
            if t2 > EPSILON {
                let x = px + dx * t2;
                let y = py + dy * t2;
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

    fn is_inside(&self, (px, py): (f64, f64)) -> bool {
        let x = px - self.cx;
        let y = py - self.cy;
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
    fn intersect(&self, (px, py): (f64, f64), (dx, dy): (f64, f64)) -> Vec<Intersection> {
        let mut result: Vec<Intersection> = Vec::new();
        let a = dx * self.nx + dy * self.ny;
        if a.abs() < EPSILON {
            result
        } else {
            let b = (self.px - px) * self.nx + (self.py - py) * self.ny;
            let t = b / a;
            if t > EPSILON {
                result.push(Intersection {
                    point: (px + dx * t, py + dy * t),
                    normal: (self.nx, self.ny),
                });
            }
            result
        }
    }

    fn is_inside(&self, (px, py): (f64, f64)) -> bool {
        (px - self.px) * self.nx + (py - self.py) * self.ny < 0.0
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
        let theta = -2.0 * PI * elevation / WHOLE_ANGLE;
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

    pub fn star(cx: f64, cy: f64, r: f64, n: u32, e: f64) -> Self {
        assert!(n >= 5);
        let mut elevation = e;
        while elevation < 0.0 {
            elevation += WHOLE_ANGLE;
        }
        while elevation >= WHOLE_ANGLE {
            elevation -= WHOLE_ANGLE
        }
        let cos = (PI / n as f64).cos();
        let scaling_ratio = (cos * cos * 2.0 - 1.0) / cos;
        Self::new((0..2 * n).map(|i| (i, i as f64 * PI / n as f64))
            .map(|(i, theta)| (i, theta + 2.0 * PI * elevation / WHOLE_ANGLE))
            .map(|(i, theta)| {
                let mut l = r;
                if i % 2 == 1 {
                    l = l * scaling_ratio;
                }
                (l * theta.cos(), l * theta.sin())
            })
            .map(|(x, y)| (cx + x, cy - y))
            .collect())
    }
}

impl Shape for Polygon {
    fn intersect(&self, (px, py): (f64, f64), (dx, dy): (f64, f64)) -> Vec<Intersection> {
        let mut result: Vec<Intersection> = Vec::new();
        for i in 0..self.points.len() {
            let a = self.points[i];
            let b = if i + 1 == self.points.len() {
                self.points[0]
            } else {
                self.points[i + 1]
            };
            let ax = a.0 - px;
            let ay = a.1 - py;
            let bx = b.0 - px;
            let by = b.1 - py;
            let product1 = ax * dy - dx * ay;
            let product2 = bx * dy - dx * by;
            if product1 * product2 < 0.0 {
                let nx = a.1 - b.1;
                let ny = b.0 - a.0;
                let len = (nx * nx + ny * ny).sqrt();
                let nx = nx / len;
                let ny = ny / len;
                let c1 = dx * nx + dy * ny;
                if c1.abs() > EPSILON {
                    let c2 = (a.0 - px) * nx + (a.1 - py) * ny;
                    let t = c2 / c1;
                    if t > EPSILON {
                        result.push(Intersection {
                            point: (px + dx * t, py + dy * t),
                            normal: (nx, ny),
                        });
                    }
                }
            }
        }
        result
    }

    fn is_inside(&self, (px, py): (f64, f64)) -> bool {
        let mut cross_count = 0;
        for i in 0..self.points.len() {
            let (x0, y0) = self.points[i];
            let mut j = i + 1;
            if j >= self.points.len() {
                j = 0
            }
            let (x1, y1) = self.points[j];

            if (x1 - x0).abs() < EPSILON {
                if (px - x0) * (px - x1) > 0.0 {
                    continue;
                }
                cross_count += 1;
                continue;
            }

            let slope = (y1 - y0) as f64 / (x1 - x0) as f64;
            let cond1 = (x0 <= px) && (px < x1);
            let cond2 = (x1 <= px) && (px < x0);
            let above = py < slope * (px - x0) + y0;
            if (cond1 || cond2) && above {
                cross_count += 1;
            }
        }
        cross_count % 2 != 0
    }
}

#[allow(dead_code)]
pub struct UnionShape {
    pub c: Vec<Box<dyn Shape + Sync>>,
}

impl Shape for UnionShape {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Vec<Intersection> {
        let length = self.c.len();
        let mut result: Vec<Intersection> = Vec::new();

        if length == 0 {
            return result;
        }

        if length == 1 {
            return self.c[0].intersect(p, d);
        }

        for i in 0..length {
            for item in self.c[i].intersect(p, d) {
                let mut check = true;
                for j in 0..length {
                    if i == j {
                        continue;
                    }
                    if self.c[j].is_inside(item.point) {
                        check = false;
                        break;
                    }
                }
                if check {
                    result.push(item);
                }
            }
        }
        result
    }

    fn is_inside(&self, p: (f64, f64)) -> bool {
        let mut result = false;
        self.c.iter().for_each(|item| {
            if item.is_inside(p) {
                result = true;
            }
        });
        result
    }
}

#[allow(dead_code)]
pub struct IntersectShape {
    pub c: Vec<Box<dyn Shape + Sync>>,
}

impl Shape for IntersectShape {
    fn intersect(&self, p: (f64, f64), d: (f64, f64)) -> Vec<Intersection> {
        let length = self.c.len();
        let mut result: Vec<Intersection> = Vec::new();

        if length == 0 {
            return result;
        }

        if length == 1 {
            return self.c[0].intersect(p, d);
        }

        for i in 0..length {
            for item in self.c[i].intersect(p, d) {
                let mut check = true;
                for j in 0..length {
                    if i == j {
                        continue;
                    }
                    if !self.c[j].is_inside(item.point) {
                        check = false;
                        break;
                    }
                }
                if check {
                    result.push(item);
                }
            }
        }
        result
    }

    fn is_inside(&self, p: (f64, f64)) -> bool {
        let mut result = true;
        self.c.iter().for_each(|item| {
            if !item.is_inside(p) {
                result = false;
            }
        });
        result
    }
}

#[allow(dead_code)]
pub struct ComplementShape {
    pub a: Box<dyn Shape + Sync>,
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
    fn new(a: Box<dyn Shape + Sync>) -> ComplementShape {
        ComplementShape {
            a
        }
    }
}