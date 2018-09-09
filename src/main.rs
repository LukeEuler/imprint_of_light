extern crate image;
extern crate rand;
extern crate rayon;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod element;
mod calculate;
mod shapes;
mod render;

use std::fs::File;

use element::Color;
use shapes::*;
use render::render;
use render::{Entity, Scene};


#[derive(Serialize, Deserialize)]
struct Config {
    enable: bool,
    out: String,
    width: u32,
    height: u32,
    stratification: u32,
    max_depth: u32,
    scenes: Vec<EntityJson>,
}

#[derive(Serialize, Deserialize)]
struct EntityJson {
    shape: ShapeJson,
    emissive: ColorJson,
    reflectivity: f64,
    eta: f64,
    absorption: ColorJson,
}

#[derive(Serialize, Deserialize)]
enum ShapeJson {
    #[serde(rename = "polygon")]
    Polygon(PolygonJson),
    #[serde(rename = "circle")]
    Circle(CircleJson),
    #[serde(rename = "plane")]
    Plane { px: f64, py: f64, nx: f64, ny: f64 },
    #[serde(rename = "union")]
    Union { a: Box<ShapeJson>, b: Box<ShapeJson> },
    #[serde(rename = "intersect")]
    Intersect { a: Box<ShapeJson>, b: Box<ShapeJson> },
}

#[derive(Serialize, Deserialize)]
enum PolygonJson {
    #[serde(rename = "points")]
    Points(Vec<(f64, f64)>),
    #[serde(rename = "regular")]
    Regular { cx: f64, cy: f64, r: f64, n: u32 },
}

#[derive(Serialize, Deserialize)]
struct CircleJson {
    cx: f64,
    cy: f64,
    r: f64,
}

#[derive(Serialize, Deserialize)]
enum ColorJson {
    #[serde(rename = "grey")]
    Grey(f64),
    #[serde(rename = "black")]
    Black(bool),
    #[serde(rename = "rgb")]
    Rgb { r: f64, g: f64, b: f64 },
}

#[allow(dead_code)]
fn get_color(color_json: ColorJson) -> Color {
    match color_json {
        ColorJson::Grey(n) => Color::grey(n),
        ColorJson::Black(_) => Color::black(),
        ColorJson::Rgb { r, g, b } => Color { r, g, b },
    }
}

#[allow(dead_code)]
fn get_shape(shape_json: ShapeJson) -> Box<Shape + Sync> {
    let shape: Box<Shape + Sync> = match shape_json {
        ShapeJson::Polygon(mut pj) => {
            match pj {
                PolygonJson::Points(points) => {
                    Box::new(Polygon::new(points))
                }
                PolygonJson::Regular { cx, cy, r, n } => {
                    Box::new(Polygon::ngon(cx, cy, r, n))
                }
            }
        }
        ShapeJson::Circle(cj) => {
            Box::new(Circle {
                cx: cj.cx,
                cy: cj.cy,
                r: cj.r,
            })
        }
        ShapeJson::Plane { px, py, nx, ny } => {
            Box::new(Plane {
                px,
                py,
                nx,
                ny,
            })
        }
        ShapeJson::Union { a, b } => {
            Box::new(UnionShape {
                a: get_shape(*a),
                b: get_shape(*b),
            })
        }
        ShapeJson::Intersect { a, b } => {
            Box::new(IntersectShape {
                a: get_shape(*a),
                b: get_shape(*b),
            })
        }
    };
    shape
}

fn main() {
    let file = File::open("config.json").unwrap();
    let configs: Vec<Config> = serde_json::from_reader(file).unwrap();

    for item in configs {
        if !item.enable {
            continue;
        }
        if item.scenes.len() == 0 {
            continue;
        }

        let mut entities: Vec<Entity> = Vec::new();
        for entity_json in item.scenes {
            let mut entity = Entity {
                shape: get_shape(entity_json.shape),
                emissive: get_color(entity_json.emissive),
                reflectivity: entity_json.reflectivity,
                eta: entity_json.eta,
                absorption: get_color(entity_json.absorption),
            };
            entities.push(entity);
        };
        let scene = Scene {
            entities,
        };
        let img = render(&scene, (item.width, item.height), item.stratification, item.max_depth);
        img.save(item.out.clone()).unwrap();
    }
}
