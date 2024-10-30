use scamper_macros::{function, ForeignValue};

use super::{color::Color, Rgb};
use crate::{
    ast::{Contract, FromValue, List, NonNegative, Value},
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

    // extended functions
    env.register("solid-square", solid_square);
    env.register("outlined-square", outlined_square);
    env.register("solid-rectangle", solid_rectangle);
    env.register("outlined-rectangle", outlined_rectangle);
    env.register("solid-circle", solid_circle);
    env.register("outlined-circle", outlined_circle);
    env.register("solid-ellipse", solid_ellipse);
    env.register("outlined-ellipse", outlined_ellipse);
    env.register("solid-triangle", solid_triangle);
    env.register("outlined-triangle", outlined_triangle);
    env.register("solid-isosceles-triangle", solid_isosceles_triangle);
    env.register("outlined-isosceles-triangle", outlined_isosceles_triangle);

    env.register("image-width", image_width);
    env.register("image-height", image_height);
    env.register("image-color", image_color);
    env.register("image-recolor", image_recolor);
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

    pub fn color(&self) -> Rgb {
        let drawings = match self {
            Drawing::Ellipse(e) => return e.color.to_rgb(),
            Drawing::Rectangle(r) => return r.color.to_rgb(),
            Drawing::Triangle(t) => return t.color.to_rgb(),
            Drawing::Path(p) => return p.color.to_rgb(),
            Drawing::Beside(b) => b.drawings.clone(),
            Drawing::Above(a) => a.drawings.clone(),
            Drawing::Overlay(o) => o.drawings.clone(),
            Drawing::OverlayOffset(o) => vec![*o.drawing1.clone(), *o.drawing2.clone()],
            Drawing::Rotate(r) => return r.drawing.color(),
            Drawing::WithDash(d) => return d.drawing.color(),
        };

        let mut avg = drawings
            .first()
            .map_or(Rgb::new(0.0, 0.0, 0.0), |d| d.color());
        for d in drawings.iter().skip(1) {
            avg = avg.average(d.color())
        }
        avg
    }
}

#[function]
fn drawing_q(x: Value) -> bool {
    Drawing::from_value(&x).is_some()
}

#[function(contract(0, NonNegative), contract(1, NonNegative))]
fn ellipse(width: f64, height: f64, mode: Mode, color: Color) -> Drawing {
    Drawing::Ellipse(Shape {
        width,
        height,
        mode,
        color,
    })
}

#[function(contract(0, NonNegative))]
fn circle(radius: f64, mode: Mode, color: Color) -> Drawing {
    let diameter = radius * 2.0;
    Drawing::Ellipse(Shape {
        width: diameter,
        height: diameter,
        mode,
        color,
    })
}

#[function(contract(0, NonNegative), contract(1, NonNegative))]
fn rectangle(width: f64, height: f64, mode: Mode, color: Color) -> Drawing {
    Drawing::Rectangle(Shape {
        width,
        height,
        mode,
        color,
    })
}

#[function(contract(0, NonNegative))]
fn square(size: f64, mode: Mode, color: Color) -> Drawing {
    Drawing::Rectangle(Shape {
        width: size,
        height: size,
        mode,
        color,
    })
}

#[function(contract(0, NonNegative))]
fn triangle(length: f64, mode: Mode, color: Color) -> Drawing {
    Drawing::Triangle(Shape {
        width: length,
        height: length * f64::sqrt(3.0) / 2.0,
        mode,
        color,
    })
}

#[function(contract(0, NonNegative), contract(1, NonNegative))]
fn isosceles_triangle(width: f64, height: f64, mode: Mode, color: Color) -> Drawing {
    Drawing::Triangle(Shape {
        width,
        height,
        mode,
        color,
    })
}

#[function(contract(0, NonNegative), contract(1, NonNegative))]
fn path(
    width: f64,
    height: f64,
    points: List,
    mode: Mode,
    color: Color,
) -> Result<Drawing, RuntimeError> {
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

fn beside_above_prim(beside: bool, align: Align, drawings: Vec<Drawing>) -> BesideAbove {
    BesideAbove {
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
    }
}

fn beside_prim(align: Align, drawings: Vec<Drawing>) -> Drawing {
    Drawing::Beside(beside_above_prim(true, align, drawings))
}

fn above_prim(align: Align, drawings: Vec<Drawing>) -> Drawing {
    Drawing::Above(beside_above_prim(false, align, drawings))
}

#[function]
fn beside(drawings: &[Drawing]) -> Drawing {
    beside_prim(Align::Center, drawings.to_vec())
}

#[function(contract(0, VAlign))]
fn beside_align(align: Align, drawings: &[Drawing]) -> Drawing {
    beside_prim(align, drawings.to_vec())
}

#[function]
fn above(drawings: &[Drawing]) -> Drawing {
    above_prim(Align::Middle, drawings.to_vec())
}

#[function(contract(0, HAlign))]
fn above_align(align: Align, drawings: &[Drawing]) -> Drawing {
    above_prim(align, drawings.to_vec())
}

fn overlay_align_prim(x_align: Align, y_align: Align, drawings: Vec<Drawing>) -> Drawing {
    Drawing::Overlay(Overlay {
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
    })
}

#[function]
fn overlay(drawings: &[Drawing]) -> Drawing {
    overlay_align_prim(Align::Middle, Align::Center, drawings.to_vec())
}

#[function(contract(0, HAlign), contract(1, VAlign))]
fn overlay_align(x_align: Align, y_align: Align, drawings: &[Drawing]) -> Drawing {
    overlay_align_prim(x_align, y_align, drawings.to_vec())
}

#[function]
fn overlay_offset(dx: f64, dy: f64, d1: Drawing, d2: Drawing) -> Drawing {
    // todo: (from upstream) what if d2 is actually bigger than d1? Then the calculation needs to mirror!
    Drawing::OverlayOffset(OverlayOffset {
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
    })
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

#[function]
fn solid_square(length: f64, color: Color) -> Drawing {
    Drawing::Rectangle(Shape {
        width: length,
        height: length,
        mode: Mode::Solid,
        color,
    })
}

#[function(contract(0, NonNegative))]
fn outlined_square(length: f64, color: Color) -> Drawing {
    Drawing::Rectangle(Shape {
        width: length,
        height: length,
        mode: Mode::Outline,
        color,
    })
}

#[function(contract(0, NonNegative), contract(1, NonNegative))]
fn solid_rectangle(width: f64, height: f64, color: Color) -> Drawing {
    Drawing::Rectangle(Shape {
        width,
        height,
        mode: Mode::Solid,
        color,
    })
}

#[function(contract(0, NonNegative), contract(1, NonNegative))]
fn outlined_rectangle(width: f64, height: f64, color: Color) -> Drawing {
    Drawing::Rectangle(Shape {
        width,
        height,
        mode: Mode::Outline,
        color,
    })
}

#[function(contract(0, NonNegative))]
fn solid_circle(radius: f64, color: Color) -> Drawing {
    let diameter = radius * 2.0;
    Drawing::Ellipse(Shape {
        width: diameter,
        height: diameter,
        mode: Mode::Solid,
        color,
    })
}

#[function(contract(0, NonNegative))]
fn outlined_circle(radius: f64, color: Color) -> Drawing {
    let diameter = radius * 2.0;
    Drawing::Ellipse(Shape {
        width: diameter,
        height: diameter,
        mode: Mode::Outline,
        color,
    })
}

#[function(contract(0, NonNegative), contract(1, NonNegative))]
fn solid_ellipse(width: f64, height: f64, color: Color) -> Drawing {
    Drawing::Ellipse(Shape {
        width,
        height,
        mode: Mode::Solid,
        color,
    })
}

#[function(contract(0, NonNegative), contract(1, NonNegative))]
fn outlined_ellipse(width: f64, height: f64, color: Color) -> Drawing {
    Drawing::Ellipse(Shape {
        width,
        height,
        mode: Mode::Outline,
        color,
    })
}

#[function(contract(0, NonNegative))]
fn solid_triangle(length: f64, color: Color) -> Drawing {
    Drawing::Triangle(Shape {
        width: length,
        height: length * f64::sqrt(3.0) / 2.0,
        mode: Mode::Solid,
        color,
    })
}

#[function(contract(0, NonNegative))]
fn outlined_triangle(length: f64, color: Color) -> Drawing {
    Drawing::Triangle(Shape {
        width: length,
        height: length * f64::sqrt(3.0) / 2.0,
        mode: Mode::Outline,
        color,
    })
}

#[function(contract(0, NonNegative), contract(1, NonNegative))]
fn solid_isosceles_triangle(width: f64, height: f64, color: Color) -> Drawing {
    Drawing::Triangle(Shape {
        width,
        height,
        mode: Mode::Solid,
        color,
    })
}

#[function(contract(0, NonNegative), contract(1, NonNegative))]
fn outlined_isosceles_triangle(width: f64, height: f64, color: Color) -> Drawing {
    Drawing::Triangle(Shape {
        width,
        height,
        mode: Mode::Outline,
        color,
    })
}

#[function]
fn image_width(drawing: Drawing) -> f64 {
    drawing.width()
}

#[function]
fn image_height(drawing: Drawing) -> f64 {
    drawing.height()
}

#[function]
fn image_color(drawing: Drawing) -> Rgb {
    drawing.color()
}

fn image_recolor_prim(drawing: Drawing, color: Color) -> Drawing {
    match drawing {
        Drawing::Rectangle(s) => Drawing::Rectangle(Shape { color, ..s }),
        Drawing::Ellipse(s) => Drawing::Ellipse(Shape { color, ..s }),
        Drawing::Triangle(s) => Drawing::Triangle(Shape { color, ..s }),
        Drawing::Path(p) => Drawing::Path(Path { color, ..p }),
        Drawing::Above(a) => Drawing::Above(BesideAbove {
            drawings: a
                .drawings
                .into_iter()
                .map(|d| image_recolor_prim(d, color.clone()))
                .collect(),
            ..a
        }),
        Drawing::Beside(b) => Drawing::Beside(BesideAbove {
            drawings: b
                .drawings
                .into_iter()
                .map(|d| image_recolor_prim(d, color.clone()))
                .collect(),
            ..b
        }),
        Drawing::Overlay(o) => Drawing::Overlay(Overlay {
            drawings: o
                .drawings
                .into_iter()
                .map(|d| image_recolor_prim(d, color.clone()))
                .collect(),
            ..o
        }),
        Drawing::OverlayOffset(o) => Drawing::OverlayOffset(OverlayOffset {
            drawing1: Box::new(image_recolor_prim(*o.drawing1, color.clone())),
            drawing2: Box::new(image_recolor_prim(*o.drawing2, color.clone())),
            ..o
        }),
        Drawing::Rotate(r) => Drawing::Rotate(Rotate {
            drawing: Box::new(image_recolor_prim(*r.drawing, color)),
            ..r
        }),
        Drawing::WithDash(w) => Drawing::WithDash(WithDash {
            drawing: Box::new(image_recolor_prim(*w.drawing, color)),
            ..w
        }),
    }
}

#[function]
fn image_recolor(drawing: Drawing, color: Color) -> Drawing {
    image_recolor_prim(drawing, color)
}
