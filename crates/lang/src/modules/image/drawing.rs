use scamper_macros::{function, ForeignValue};

use super::color::Color;
use crate::{
    ast::{Contract, FromValue, List, Value},
    interpreter::{Env, RuntimeError},
};

pub fn add_to(env: &mut Env) {
    env.register("image?", drawing_q);
    env.register("ellipse", ellipse);
    env.register("circle", circle);
    env.register("rectangle", rectangle);
    env.register("square", square);
    env.register("triangle", triangle);
    env.register("isosceles-triangle", isosceles_triangle);
    env.register("path", path);
    env.register("beside", beside);
    env.register("beside/align", beside_align);
    env.register("above", above);
    env.register("above/align", above_align);
    env.register("overlay", overlay);
    env.register("overlay/align", overlay_align);
    env.register("overlay/offset", overlay_offset);
    env.register("rotate", rotate);
    env.register("with-dash", with_dash);
}

#[derive(Debug, Clone)]
pub enum Mode {
    Solid,
    Outline,
}

impl FromValue for Mode {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::String(s) => match s.as_ref() {
                "solid" => Some(Mode::Solid),
                "outline" => Some(Mode::Outline),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Align {
    Top,
    Bottom,
    Middle,
    Left,
    Right,
    Center,
}

impl FromValue for Align {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::String(s) => match s.as_ref() {
                "top" => Some(Align::Top),
                "bottom" => Some(Align::Bottom),
                "middle" => Some(Align::Middle),
                "left" => Some(Align::Left),
                "right" => Some(Align::Right),
                "center" => Some(Align::Center),
                _ => None,
            },
            _ => None,
        }
    }
}

struct HAlign;
impl Contract for HAlign {
    fn check(&self, value: &Value) -> bool {
        value
            .string()
            .map_or(false, |s| s == "left" || s == "middle" || s == "right")
    }

    fn name(&self) -> &'static str {
        "horizontal alignment (\"left\", \"middle\", or \"right\")"
    }
}

struct VAlign;
impl Contract for VAlign {
    fn check(&self, value: &Value) -> bool {
        value
            .string()
            .map_or(false, |s| s == "top" || s == "center" || s == "bottom")
    }

    fn name(&self) -> &'static str {
        "vertical alignment (\"top\", \"center\", or \"bottom\")"
    }
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub width: f64,
    pub height: f64,
    pub mode: Mode,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct Path {
    pub width: f64,
    pub height: f64,
    pub points: Vec<(f64, f64)>,
    pub mode: Mode,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct BesideAbove {
    pub width: f64,
    pub height: f64,
    pub align: Align,
    pub drawings: Vec<Drawing>,
}

#[derive(Debug, Clone)]
pub struct Overlay {
    pub width: f64,
    pub height: f64,
    pub x_align: Align,
    pub y_align: Align,
    pub drawings: Vec<Drawing>,
}

#[derive(Debug, Clone)]
pub struct OverlayOffset {
    pub width: f64,
    pub height: f64,
    pub dx: f64,
    pub dy: f64,
    pub drawing1: Box<Drawing>,
    pub drawing2: Box<Drawing>,
}

#[derive(Debug, Clone)]
pub struct Rotate {
    pub width: f64,
    pub height: f64,
    pub angle: f64,
    pub drawing: Box<Drawing>,
}

#[derive(Debug, Clone)]
pub struct WithDash {
    pub width: f64,
    pub height: f64,
    pub dash_spec: Vec<f64>,
    pub drawing: Box<Drawing>,
}

#[derive(Debug, Clone, ForeignValue)]
pub enum Drawing {
    Ellipse(Shape),
    Rectangle(Shape),
    Triangle(Shape),
    Path(Path),
    Beside(BesideAbove),
    Above(BesideAbove),
    Overlay(Overlay),
    OverlayOffset(OverlayOffset),
    Rotate(Rotate),
    WithDash(WithDash),
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
            Drawing::Overlay(o) => o.width,
            Drawing::OverlayOffset(o) => o.width,
            Drawing::Rotate(r) => r.width,
            Drawing::WithDash(d) => d.width,
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
            Drawing::Overlay(o) => o.height,
            Drawing::OverlayOffset(o) => o.height,
            Drawing::Rotate(r) => r.height,
            Drawing::WithDash(d) => d.height,
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
    mode: Mode,
    color: Color,
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
        mode,
        color,
    }))
}

#[function]
fn ellipse(width: f64, height: f64, mode: Mode, color: Color) -> Result<Drawing, RuntimeError> {
    ellipse_prim(width, height, mode, color)
}

#[function]
fn circle(radius: f64, mode: Mode, color: Color) -> Result<Drawing, RuntimeError> {
    ellipse_prim(radius * 2.0, radius * 2.0, mode, color)
}

fn rectangle_prim(
    width: f64,
    height: f64,
    mode: Mode,
    color: Color,
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
        mode,
        color,
    }))
}

#[function]
fn rectangle(width: f64, height: f64, mode: Mode, color: Color) -> Result<Drawing, RuntimeError> {
    rectangle_prim(width, height, mode, color)
}

#[function]
fn square(size: f64, mode: Mode, color: Color) -> Result<Drawing, RuntimeError> {
    rectangle_prim(size, size, mode, color)
}

fn triangle_prim(
    width: f64,
    height: f64,
    mode: Mode,
    color: Color,
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
        mode,
        color,
    }))
}

#[function]
fn triangle(length: f64, mode: Mode, color: Color) -> Result<Drawing, RuntimeError> {
    triangle_prim(length, length * f64::sqrt(3.0) / 2.0, mode, color)
}

#[function]
fn isosceles_triangle(
    width: f64,
    height: f64,
    mode: Mode,
    color: Color,
) -> Result<Drawing, RuntimeError> {
    triangle_prim(width, height, mode, color)
}

#[function]
fn path(
    width: f64,
    height: f64,
    points: List,
    mode: Mode,
    color: Color,
) -> Result<Drawing, RuntimeError> {
    if width <= 0.0 || height <= 0.0 {
        return Err(RuntimeError::new(
            "Invalid width or height".to_string(),
            None,
        ));
    }
    let points = points
        .values()
        .iter()
        .map(|v| {
            match v {
                Value::Pair(px, py) => match (px.numeric(), py.numeric()) {
                    (Some(x), Some(y)) => return Ok((x, y)),
                    _ => {}
                },
                Value::List(l) if l.len() == 2 => match (l[0].numeric(), l[1].numeric()) {
                    (Some(x), Some(y)) => return Ok((x, y)),
                    _ => {}
                },
                _ => {}
            }
            Err(RuntimeError::new(
                "Each point must be a list of two numbers".to_string(),
                None,
            ))
        })
        .collect::<Result<Vec<(f64, f64)>, RuntimeError>>()?;
    Ok(Drawing::Path(Path {
        width,
        height,
        points,
        mode,
        color,
    }))
}

fn beside_above_prim(
    beside: bool,
    align: Align,
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
        align,
        drawings,
    })
}

fn beside_prim(align: Align, drawings: Vec<Drawing>) -> Result<Drawing, RuntimeError> {
    Ok(Drawing::Beside(beside_above_prim(true, align, drawings)?))
}

#[function]
fn beside(drawings: &[Drawing]) -> Result<Drawing, RuntimeError> {
    beside_prim(Align::Center, drawings.to_vec())
}

#[function]
fn beside_align(align: Align, drawings: &[Drawing]) -> Result<Drawing, RuntimeError> {
    beside_prim(align, drawings.to_vec())
}

fn above_prim(align: Align, drawings: Vec<Drawing>) -> Result<Drawing, RuntimeError> {
    Ok(Drawing::Above(beside_above_prim(false, align, drawings)?))
}

#[function]
fn above(drawings: &[Drawing]) -> Result<Drawing, RuntimeError> {
    above_prim(Align::Middle, drawings.to_vec())
}

#[function]
fn above_align(align: Align, drawings: &[Drawing]) -> Result<Drawing, RuntimeError> {
    above_prim(align, drawings.to_vec())
}

fn overlay_align_prim(
    x_align: Align,
    y_align: Align,
    drawings: Vec<Drawing>,
) -> Result<Drawing, RuntimeError> {
    Ok(Drawing::Overlay(Overlay {
        width: drawings
            .iter()
            .map(|d| d.width())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0),
        height: drawings
            .iter()
            .map(|d| d.height())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0),
        x_align,
        y_align,
        drawings,
    }))
}

#[function]
fn overlay(drawings: &[Drawing]) -> Result<Drawing, RuntimeError> {
    overlay_align_prim(Align::Middle, Align::Center, drawings.to_vec())
}

#[function(contract(0, HAlign), contract(1, VAlign))]
fn overlay_align(
    x_align: Align,
    y_align: Align,
    drawings: &[Drawing],
) -> Result<Drawing, RuntimeError> {
    overlay_align_prim(x_align, y_align, drawings.to_vec())
}

#[function]
fn overlay_offset(dx: f64, dy: f64, d1: Drawing, d2: Drawing) -> Result<Drawing, RuntimeError> {
    // todo: (from upstream) what if d2 is actually bigger than d1? Then the calculation needs to mirror!
    Ok(Drawing::OverlayOffset(OverlayOffset {
        width: if d1.width() > d2.width() {
            if dx >= 0.0 {
                f64::max(d1.width(), d2.width() + f64::abs(dx))
            } else {
                d1.width() + f64::abs(dx)
            }
        } else {
            if dx <= 0.0 {
                f64::max(d2.width(), d1.width() + f64::abs(dx))
            } else {
                d2.width() + f64::abs(dx)
            }
        },
        height: if d1.height() > d2.height() {
            if dy >= 0.0 {
                f64::max(d1.height(), d2.height() + f64::abs(dy))
            } else {
                d1.height() + f64::abs(dy)
            }
        } else {
            if dy <= 0.0 {
                f64::max(d2.height(), d1.height() + f64::abs(dy))
            } else {
                d2.height() + f64::abs(dy)
            }
        },
        dx,
        dy,
        drawing1: Box::new(d1),
        drawing2: Box::new(d2),
    }))
}

#[function]
fn rotate(angle: f64, drawing: Drawing) -> Drawing {
    let angle_rad = angle.to_radians();
    let orig_points = vec![
        (-drawing.width() / 2.0, -drawing.height() / 2.0),
        (drawing.width() / 2.0, -drawing.height() / 2.0),
        (-drawing.width() / 2.0, drawing.height() / 2.0),
        (drawing.width() / 2.0, drawing.height() / 2.0),
    ];
    let rotated_points = orig_points
        .iter()
        .map(|(x, y)| {
            (
                x * angle_rad.cos() - y * angle_rad.sin(),
                x * angle_rad.sin() + y * angle_rad.cos(),
            )
        })
        .collect::<Vec<(f64, f64)>>();

    let x_min = rotated_points
        .iter()
        .map(|(x, _)| *x)
        .fold(f64::INFINITY, f64::min);
    let x_max = rotated_points
        .iter()
        .map(|(x, _)| *x)
        .fold(f64::NEG_INFINITY, f64::max);
    let y_min = rotated_points
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::INFINITY, f64::min);
    let y_max = rotated_points
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::NEG_INFINITY, f64::max);

    let width = x_max - x_min;
    let height = y_max - y_min;

    Drawing::Rotate(Rotate {
        width,
        height,
        angle,
        drawing: Box::new(drawing),
    })
}

#[function]
fn with_dash(dash_spec: List, drawing: Drawing) -> Result<Drawing, RuntimeError> {
    let dash_spec = dash_spec
        .values()
        .iter()
        .map(|v| match v.numeric() {
            Some(n) => Ok(n),
            _ => Err(RuntimeError::new(
                "dash spec must be a list of numbers".to_string(),
                None,
            )),
        })
        .collect::<Result<Vec<f64>, RuntimeError>>()?;
    Ok(Drawing::WithDash(WithDash {
        width: drawing.width(),
        height: drawing.height(),
        dash_spec,
        drawing: Box::new(drawing),
    }))
}
