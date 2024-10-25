use scamper_macros::{function, ForeignValue};

use super::color::Rgb;
use crate::{
    ast::{FromValue, Value},
    interpreter::{Env, RuntimeError},
};

pub fn add_to(env: &mut Env) {
    env.register("drawing?", drawing_q);
    env.register("ellipse", ellipse);
    env.register("circle", circle);
    env.register("rectangle", rectangle);
    env.register("square", square);
    env.register("triangle", triangle);
    env.register("isosceles-triangle", isosceles_triangle);
    // env.register("path", path);
    env.register("beside", beside);
    env.register("beside/align", beside_align);
    env.register("above", above);
    env.register("above/align", above_align);
}

#[derive(Debug, Clone)]
pub enum Mode {
    Solid,
    Outline,
}

#[derive(Debug, Clone)]
pub enum Align {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub width: f64,
    pub height: f64,
    pub mode: Mode,
    pub color: Rgb,
}

#[derive(Debug, Clone)]
pub struct Path {
    pub width: f64,
    pub height: f64,
    pub points: Vec<(f64, f64)>,
    pub mode: Mode,
    pub color: Rgb,
}

#[derive(Debug, Clone)]
pub struct BesideAbove {
    pub width: f64,
    pub height: f64,
    pub align: Align,
    pub drawings: Vec<Drawing>,
}

#[derive(Debug, Clone, ForeignValue)]
pub enum Drawing {
    Ellipse(Shape),
    Rectangle(Shape),
    Triangle(Shape),
    Path(Path),
    Beside(BesideAbove),
    Above(BesideAbove),
}

impl Drawing {
    pub fn width(&self) -> f64 {
        match self {
            Drawing::Ellipse(e) => e.width,
            Drawing::Rectangle(r) => r.width,
            Drawing::Triangle(t) => t.width,
            Drawing::Path(p) => p.width,
            Drawing::Beside(b) => b.width,
            Drawing::Above(a) => a.width,
        }
    }

    pub fn height(&self) -> f64 {
        match self {
            Drawing::Ellipse(e) => e.height,
            Drawing::Rectangle(r) => r.height,
            Drawing::Triangle(t) => t.height,
            Drawing::Path(p) => p.height,
            Drawing::Beside(b) => b.height,
            Drawing::Above(a) => a.height,
        }
    }
}

#[function]
fn drawing_q(x: Value) -> bool {
    Drawing::from_value(&x).is_some()
}

fn ellipse_prim(
    width: f64,
    height: f64,
    mode: String,
    color: Rgb,
) -> Result<Drawing, RuntimeError> {
    if width <= 0.0 || height <= 0.0 {
        return Err(RuntimeError::new(
            "Invalid width or height".to_string(),
            None,
        ));
    }
    Ok(Drawing::Ellipse(Shape {
        width,
        height,
        mode: match mode.as_str() {
            "solid" => Mode::Solid,
            "outline" => Mode::Outline,
            _ => return Err(RuntimeError::new("Invalid mode".to_string(), None)),
        },
        color,
    }))
}

#[function]
fn ellipse(width: f64, height: f64, mode: String, color: Rgb) -> Result<Drawing, RuntimeError> {
    ellipse_prim(width, height, mode, color)
}

#[function]
fn circle(radius: f64, mode: String, color: Rgb) -> Result<Drawing, RuntimeError> {
    ellipse_prim(radius * 2.0, radius * 2.0, mode, color)
}

fn rectangle_prim(
    width: f64,
    height: f64,
    mode: String,
    color: Rgb,
) -> Result<Drawing, RuntimeError> {
    if width <= 0.0 || height <= 0.0 {
        return Err(RuntimeError::new(
            "Invalid width or height".to_string(),
            None,
        ));
    }
    Ok(Drawing::Rectangle(Shape {
        width,
        height,
        mode: match mode.as_str() {
            "solid" => Mode::Solid,
            "outline" => Mode::Outline,
            _ => return Err(RuntimeError::new("Invalid mode".to_string(), None)),
        },
        color,
    }))
}

#[function]
fn rectangle(width: f64, height: f64, mode: String, color: Rgb) -> Result<Drawing, RuntimeError> {
    rectangle_prim(width, height, mode, color)
}

#[function]
fn square(size: f64, mode: String, color: Rgb) -> Result<Drawing, RuntimeError> {
    rectangle_prim(size, size, mode, color)
}

fn triangle_prim(
    width: f64,
    height: f64,
    mode: String,
    color: Rgb,
) -> Result<Drawing, RuntimeError> {
    if width <= 0.0 || height <= 0.0 {
        return Err(RuntimeError::new(
            "Invalid width or height".to_string(),
            None,
        ));
    }
    Ok(Drawing::Triangle(Shape {
        width,
        height,
        mode: match mode.as_str() {
            "solid" => Mode::Solid,
            "outline" => Mode::Outline,
            _ => return Err(RuntimeError::new("Invalid mode".to_string(), None)),
        },
        color,
    }))
}

#[function]
fn triangle(length: f64, mode: String, color: Rgb) -> Result<Drawing, RuntimeError> {
    triangle_prim(length, length * f64::sqrt(3.0) / 2.0, mode, color)
}

#[function]
fn isosceles_triangle(
    width: f64,
    height: f64,
    mode: String,
    color: Rgb,
) -> Result<Drawing, RuntimeError> {
    triangle_prim(width, height, mode, color)
}

fn beside_above_prim(
    beside: bool,
    align: &str,
    drawings: Vec<Drawing>,
) -> Result<BesideAbove, RuntimeError> {
    Ok(BesideAbove {
        width: if beside {
            drawings.iter().map(|d| d.width()).sum()
        } else {
            drawings
                .iter()
                .map(|d| d.width())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        },
        height: if beside {
            drawings
                .iter()
                .map(|d| d.height())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        } else {
            drawings.iter().map(|d| d.height()).sum()
        },
        align: match align {
            "top" => Align::Top,
            "bottom" => Align::Bottom,
            "left" => Align::Left,
            "right" => Align::Right,
            "center" => Align::Center,
            _ => return Err(RuntimeError::new("Invalid align".to_string(), None)),
        },
        drawings,
    })
}

fn beside_align_prim(align: &str, drawings: Vec<Drawing>) -> Result<Drawing, RuntimeError> {
    Ok(Drawing::Beside(beside_above_prim(true, align, drawings)?))
}

#[function]
fn beside(drawings: &[Drawing]) -> Result<Drawing, RuntimeError> {
    beside_align_prim("center", drawings.to_vec())
}

#[function]
fn beside_align(args: &[Value]) -> Result<Drawing, RuntimeError> {
    let Value::String(align) = args[0].clone() else {
        return Err(RuntimeError::new("Invalid align".to_string(), None));
    };
    let drawings = args[1..]
        .iter()
        .map(|v| {
            Drawing::from_value(v).ok_or(RuntimeError::new("Invalid drawing".to_string(), None))
        })
        .collect::<Result<Vec<_>, RuntimeError>>()?;
    beside_align_prim(&align, drawings.to_vec())
}

fn above_prim(align: &str, drawings: Vec<Drawing>) -> Result<Drawing, RuntimeError> {
    Ok(Drawing::Above(beside_above_prim(false, align, drawings)?))
}

#[function]
fn above(drawings: &[Drawing]) -> Result<Drawing, RuntimeError> {
    above_prim("center", drawings.to_vec())
}

#[function]
fn above_align(args: &[Value]) -> Result<Drawing, RuntimeError> {
    let Value::String(align) = args[0].clone() else {
        return Err(RuntimeError::new("Invalid align".to_string(), None));
    };
    let drawings = args[1..]
        .iter()
        .map(|v| {
            Drawing::from_value(v).ok_or(RuntimeError::new("Invalid drawing".to_string(), None))
        })
        .collect::<Result<Vec<_>, RuntimeError>>()?;
    above_prim(&align, drawings.to_vec())
}
