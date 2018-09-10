use crate::{element::Color, render::Entity, shapes::*};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub enable: bool,
    pub out: String,
    pub width: u32,
    pub height: u32,
    pub stratification: u32,
    pub max_depth: u32,
    pub scenes: Vec<EntityJson>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityJson {
    pub shape: ShapeJson,
    pub emissive: ColorJson,
    pub reflectivity: f64,
    pub eta: f64,
    pub absorption: ColorJson,
}

#[allow(dead_code)]
impl EntityJson {
    pub fn get_entity(self) -> Entity {
        Entity {
            shape: get_shape(self.shape),
            emissive: get_color(self.emissive),
            reflectivity: self.reflectivity,
            eta: self.eta,
            absorption: get_color(self.absorption),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ShapeJson {
    #[serde(rename = "directional_light")]
    DirectionalLight { d: f64, nx: f64, ny: f64 },
    #[serde(rename = "polygon")]
    Polygon(PolygonJson),
    #[serde(rename = "circle")]
    Circle(CircleJson),
    #[serde(rename = "plane")]
    Plane { px: f64, py: f64, nx: f64, ny: f64 },
    #[serde(rename = "union")]
    Union(Vec<Box<ShapeJson>>),
    #[serde(rename = "intersect")]
    Intersect(Vec<Box<ShapeJson>>),
    #[serde(rename = "complement")]
    Complement(Box<ShapeJson>),
}

#[derive(Serialize, Deserialize)]
pub enum PolygonJson {
    #[serde(rename = "points")]
    Points(Vec<(f64, f64)>),
    #[serde(rename = "regular")]
    Regular {
        cx: f64,
        cy: f64,
        r: f64,
        n: u32,
        e: f64,
    },
    #[serde(rename = "star")]
    Star {
        cx: f64,
        cy: f64,
        r: f64,
        n: u32,
        e: f64,
    },
    #[serde(rename = "rectangle")]
    Rectangle {
        cx: f64,
        cy: f64,
        e: f64,
        sx: f64,
        sy: f64,
    },
}

#[derive(Serialize, Deserialize)]
pub struct CircleJson {
    cx: f64,
    cy: f64,
    r: f64,
}

#[derive(Serialize, Deserialize)]
pub enum ColorJson {
    #[serde(rename = "grey")]
    Grey(f64),
    #[serde(rename = "black")]
    Black(bool),
    #[serde(rename = "rgb")]
    Rgb { r: f64, g: f64, b: f64 },
}

fn get_color(color_json: ColorJson) -> Color {
    match color_json {
        ColorJson::Grey(n) => Color::grey(n),
        ColorJson::Black(_) => Color::black(),
        ColorJson::Rgb { r, g, b } => Color { r, g, b },
    }
}

fn get_shape(shape_json: ShapeJson) -> Box<dyn Shape + Sync> {
    let shape: Box<dyn Shape + Sync> = match shape_json {
        ShapeJson::DirectionalLight { d, nx, ny } => Box::new(DirectionalLight {
            d,
            nx: -nx,
            ny: -ny,
        }),
        ShapeJson::Polygon(pj) => match pj {
            PolygonJson::Points(points) => Box::new(Polygon::new(points)),
            PolygonJson::Regular { cx, cy, r, n, e } => Box::new(Polygon::regular(cx, cy, r, n, e)),
            PolygonJson::Star { cx, cy, r, n, e } => Box::new(Polygon::star(cx, cy, r, n, e)),
            PolygonJson::Rectangle { cx, cy, e, sx, sy } => {
                Box::new(Polygon::rectangle(cx, cy, e, sx, sy))
            }
        },
        ShapeJson::Circle(cj) => Box::new(Circle {
            cx: cj.cx,
            cy: cj.cy,
            r: cj.r,
        }),
        ShapeJson::Plane { px, py, nx, ny } => Box::new(Plane { px, py, nx, ny }),
        ShapeJson::Union(list) => {
            let mut shapes: Vec<Box<dyn Shape + Sync>> = Vec::new();
            for item in list {
                let shape = get_shape(*item);
                shapes.push(shape);
            }
            Box::new(UnionShape { c: shapes })
        }
        ShapeJson::Intersect(list) => {
            let mut shapes: Vec<Box<dyn Shape + Sync>> = Vec::new();
            for item in list {
                let shape = get_shape(*item);
                shapes.push(shape);
            }
            Box::new(IntersectShape { c: shapes })
        }
        ShapeJson::Complement(a) => Box::new(ComplementShape { a: get_shape(*a) }),
    };
    shape
}
