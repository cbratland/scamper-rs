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
    env.register("font", font);
    env.register("text", text);

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

    fn name() -> &'static str {
        "mode (\"solid\" or \"outline\")"
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

    fn name() -> &'static str {
        "alignment (\"top\", \"bottom\", \"middle\", \"left\", \"right\", or \"center\")"
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
    pub x_offset: f64,
    pub y_offset: f64,
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
pub struct Font {
    pub face: String,
    pub system: Option<String>,
    pub bold: bool,
    pub italic: bool,
}

impl Font {
    pub fn new(face: String) -> Self {
        Font {
            face,
            system: Some("sans-serif".to_string()),
            bold: false,
            italic: false,
        }
    }

    pub fn to_string(&self, size: f64) -> String {
        let mut s = String::new();
        if self.bold {
            s.push_str("bold ");
        }
        if self.italic {
            s.push_str("italic ");
        }
        s.push_str(&format!("{}px \"{}\"", size, self.face));
        if let Some(system) = &self.system {
            s.push_str(&format!(", {}", system));
        }
        s
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    pub width: f64,
    pub height: f64,
    pub text: String,
    pub size: f64,
    pub color: Color,
    pub font: Font,
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
    Text(Text),
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
            Drawing::Text(t) => t.width,
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
            Drawing::Text(t) => t.height,
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
            Drawing::Text(t) => return t.color.to_rgb(),
        };

        let mut avg = drawings
            .first()
            .map_or(Rgb::new(0.0, 0.0, 0.0), |d| d.color());
        for d in drawings.iter().skip(1) {
            avg = avg.average(d.color())
        }
        avg
    }

    pub fn points(&self) -> Vec<(f64, f64)> {
        match self {
            Drawing::Ellipse(e) => {
                let mut points = Vec::new();
                let n = 100;
                for i in 0..n {
                    let t = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
                    points.push((0.5 * e.width * t.cos(), 0.5 * e.height * t.sin()));
                }
                points
            }
            Drawing::Rectangle(r) => {
                vec![
                    (0.0, 0.0),
                    (r.width, 0.0),
                    (r.width, r.height),
                    (0.0, r.height),
                ]
            }
            Drawing::Triangle(t) => {
                vec![(0.0, 0.0), (t.width, 0.0), (0.5 * t.width, t.height)]
            }
            Drawing::Path(p) => p.points.clone(),
            Drawing::Beside(b) => {
                let image_height = b.height;

                let mut points = Vec::new();
                let mut x_offset = 0.0;
                for subimage in &b.drawings {
                    points.extend(match b.align {
                        Align::Bottom => subimage
                            .points()
                            .iter()
                            .map(|(x, y)| (x + x_offset, *y + image_height - subimage.height()))
                            .collect::<Vec<_>>(),
                        Align::Top => subimage
                            .points()
                            .iter()
                            .map(|(x, y)| (x + x_offset, *y))
                            .collect::<Vec<_>>(),
                        _ => subimage
                            .points()
                            .iter()
                            .map(|(x, y)| {
                                (x + x_offset, *y + (image_height - subimage.height()) / 2.0)
                            })
                            .collect::<Vec<_>>(),
                    });

                    x_offset += subimage.width();
                }

                points
            }
            Drawing::Above(a) => {
                let image_width = a.width;
                let mut points = Vec::new();
                let mut y_offset = 0.0;
                for subimage in &a.drawings {
                    points.extend(match a.align {
                        Align::Left => subimage
                            .points()
                            .iter()
                            .map(|(x, y)| (*x, y + y_offset))
                            .collect::<Vec<_>>(),
                        Align::Right => subimage
                            .points()
                            .iter()
                            .map(|(x, y)| (*x + image_width - subimage.width(), y + y_offset))
                            .collect::<Vec<_>>(),
                        _ => subimage
                            .points()
                            .iter()
                            .map(|(x, y)| {
                                (*x + (image_width - subimage.width()) / 2.0, y + y_offset)
                            })
                            .collect::<Vec<_>>(),
                    });
                    y_offset += subimage.height();
                }
                points
            }
            Drawing::Overlay(o) => o
                .drawings
                .iter()
                .rev()
                .flat_map(|d| {
                    d.points()
                        .iter()
                        .map(|(x, y)| {
                            let x_offset = match o.x_align {
                                Align::Left => 0.0,
                                Align::Right => o.width - d.width(),
                                _ => (o.width - d.width()) / 2.0,
                            };
                            let y_offset = match o.y_align {
                                Align::Top => 0.0,
                                Align::Bottom => o.height - d.height(),
                                _ => (o.height - d.height()) / 2.0,
                            };
                            (x + x_offset, y + y_offset)
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
            Drawing::OverlayOffset(o) => {
                let x1 = if o.dx > 0.0 { 0.0 } else { f64::abs(o.dx) };
                let y1 = if o.dy > 0.0 { 0.0 } else { f64::abs(o.dy) };
                let x2 = if o.dx > 0.0 { o.dx } else { 0.0 };
                let y2 = if o.dy > 0.0 { o.dy } else { 0.0 };

                let d1_points = o.drawing1.points();
                let d2_points = o.drawing2.points();

                let d1_new_points = d1_points.iter().map(|(x, y)| (x + x1, y + y1));
                let d2_new_points = d2_points.iter().map(|(x, y)| (x + x2, y + y2));

                d1_new_points.chain(d2_new_points).collect()
            }
            Drawing::Rotate(r) => {
                let angle_rad = r.angle.to_radians();

                let rotated_points: Vec<(f64, f64)> = r
                    .drawing
                    .points()
                    .iter()
                    .map(|(x, y)| {
                        (
                            x * angle_rad.cos() - y * angle_rad.sin(),
                            x * angle_rad.sin() + y * angle_rad.cos(),
                        )
                    })
                    .collect();

                let x_min = rotated_points
                    .iter()
                    .map(|(x, _)| *x)
                    .fold(f64::INFINITY, |a, b| a.min(b));
                let y_min = rotated_points
                    .iter()
                    .map(|(_, y)| *y)
                    .fold(f64::INFINITY, |a, b| a.min(b));

                rotated_points
                    .iter()
                    .map(|(x, y)| (x - x_min, y - y_min))
                    .collect()
            }
            Drawing::WithDash(d) => d.drawing.points(),
            Drawing::Text(t) => {
                vec![
                    (0.0, 0.0),
                    (t.width, 0.0),
                    (t.width, t.height),
                    (0.0, t.height),
                ]
            }
        }
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

    let rotated_points: Vec<(f64, f64)> = drawing
        .points()
        .iter()
        .map(|(x, y)| {
            (
                x * angle_rad.cos() - y * angle_rad.sin(),
                x * angle_rad.sin() + y * angle_rad.cos(),
            )
        })
        .collect();

    let x_coords: Vec<f64> = rotated_points.iter().map(|(x, _)| *x).collect();
    let y_coords: Vec<f64> = rotated_points.iter().map(|(_, y)| *y).collect();

    let x_min = x_coords.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let x_max = x_coords.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = y_coords.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let y_max = y_coords.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let width = x_max - x_min;
    let height = y_max - y_min;

    Drawing::Rotate(Rotate {
        width,
        height,
        angle,
        x_offset: -x_min,
        y_offset: -y_min,
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
fn font(name: String, system: Option<String>, bold: Option<bool>, italic: Option<bool>) -> Font {
    Font {
        face: name,
        system: Some(system.unwrap_or("sans-serif".to_string())),
        bold: bold.unwrap_or(false),
        italic: italic.unwrap_or(false),
    }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use font_kit::{
    properties::{Properties, Style, Weight},
    source::SystemSource,
};

pub struct TextDimensions {
    pub width: f64,
    pub height: f64,
}

#[cfg(target_arch = "wasm32")]
pub fn measure_text(font: &Font, text: &str, size: f64) -> Result<TextDimensions, String> {
    use web_sys;

    let document = web_sys::window()
        .ok_or("No window found")?
        .document()
        .ok_or("No document found")?;

    let canvas = document
        .create_element("canvas")
        .ok()
        .and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        .ok_or("Failed to create canvas")?;

    let context = canvas
        .get_context("2d")
        .ok()
        .and_then(|e| e?.dyn_into::<web_sys::CanvasRenderingContext2d>().ok())
        .ok_or("Failed to get 2d context")?;

    let font_string = font.to_string(size);
    context.set_font(&font_string);

    let metrics = context
        .measure_text(text)
        .map_err(|_| "Failed to measure text")?;
    let width = metrics.width() as f64;

    let ascent = metrics.actual_bounding_box_ascent() as f64;
    let descent = metrics.actual_bounding_box_descent() as f64;
    let height = ascent + descent + 1.0;

    Ok(TextDimensions { width, height })
}

#[cfg(not(target_arch = "wasm32"))]
pub fn measure_text(font: &Font, text: &str, size: f64) -> Result<TextDimensions, String> {
    use font_kit::family_name::FamilyName;

    let source = SystemSource::new();

    let properties = Properties {
        weight: if font.bold {
            Weight::BOLD
        } else {
            Weight::NORMAL
        },
        style: if font.italic {
            Style::Italic
        } else {
            Style::Normal
        },
        ..Properties::new()
    };

    let font = source
        .select_best_match(
            &[
                FamilyName::Title(font.face.clone()),
                font.system
                    .as_ref()
                    .map(|f| FamilyName::Title(f.clone()))
                    .unwrap_or(FamilyName::SansSerif),
            ],
            &properties,
        )
        .map_err(|e| format!("Failed to select font: {}", e))?
        .load()
        .map_err(|e| format!("Failed to load font: {}", e))?;

    let metrics = font.metrics();
    let scale = size / metrics.units_per_em as f64;

    let mut width = 0.0;
    for c in text.chars() {
        if let Some(glyph_id) = font.glyph_for_char(c) {
            let advance = font
                .advance(glyph_id)
                .map_err(|e| format!("Failed to get glyph advance: {}", e))?;
            width += advance.x() as f64 * scale;
        }
    }

    let height = (metrics.ascent - metrics.descent) as f64 * scale;

    Ok(TextDimensions { width, height })
}

#[function]
fn text(
    text: String,
    size: f64,
    color: Color,
    font: Option<Font>,
) -> Result<Drawing, RuntimeError> {
    let font = font.unwrap_or_else(|| Font::new(String::from("Arial")));
    match measure_text(&font, &text, size) {
        Ok(dimensions) => Ok(Drawing::Text(Text {
            width: dimensions.width,
            height: dimensions.height,
            text,
            size,
            color,
            font,
        })),
        Err(e) => Err(RuntimeError::new(e, None)),
    }
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
        Drawing::Text(t) => Drawing::Text(Text { color, ..t }),
    }
}

#[function]
fn image_recolor(drawing: Drawing, color: Color) -> Drawing {
    image_recolor_prim(drawing, color)
}
