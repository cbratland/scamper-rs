use crate::{
    ast::{Contract, FromValue, List, Struct, Value},
    interpreter::{Env, RuntimeError},
    make_range_checker,
};
use scamper_macros::{function, ScamperStruct};
use std::fmt::Display;

pub fn add_to(env: &mut Env) {
    // rgb(a) colors
    Rgb::add_to(env);
    env.register("rgb-component?", is_rgb_component);
    env.register("rgb-distance", rgb_distance);

    // hsv colors
    Hsv::add_to(env);
    env.register("hsv-complement", hsv_complement);
    env.register("rgb-value", rgb_value);
    env.register("rgb-saturation", rgb_saturation);
    env.register("rgb-hue", rgb_hue);

    // color names
    env.register("color-name?", is_color_name);
    env.register("all-color-names", all_color_names);
    env.register("find-colors", find_colors);

    // color strings
    env.register("rgb->string", rgb_to_string);
    env.register("hsv->string", hsv_to_string);

    // color conversion
    env.register("rgb->hsv", rgb_to_hsv);
    env.register("hsv->rgb", hsv_to_rgb);
    env.register("color-name->rgb", color_name_to_rgb);

    // color transformations
    env.register("rgb-darker", rgb_darker);
    env.register("rgb-lighter", rgb_lighter);
    env.register("rgb-redder", rgb_redder);
    env.register("rgb-greener", rgb_greener);
    env.register("rgb-bluer", rgb_bluer);
    env.register("rgb-pseudo-complement", rgb_pseudo_complement);
    env.register("rgb-grayscale", rgb_grayscale);
    env.register("rgb-greyscale", rgb_grayscale);
    env.register("rgb-phaseshift", rgb_phaseshift);
    env.register("rgb-rotate-components", rgb_rotate_components);
    env.register("rgb-thin", rgb_thin);
    env.register("rgb-thicken", rgb_thicken);

    // color combinations
    env.register("rgb-add", rgb_add);
    env.register("rgb-subtract", rgb_subtract);
    env.register("rgb-average", rgb_average);
}

make_range_checker!(RgbComponentChecker, 0.0, 255.0);

#[derive(Debug, Clone, Copy, ScamperStruct)]
pub struct Rgb {
    #[contract(RgbComponentChecker)]
    pub red: f64,
    #[contract(RgbComponentChecker)]
    pub green: f64,
    #[contract(RgbComponentChecker)]
    pub blue: f64,
    #[contract(RgbComponentChecker)]
    #[default(255.0)]
    pub alpha: f64,
}

impl Rgb {
    pub const fn new(red: f64, green: f64, blue: f64) -> Rgb {
        Rgb {
            red,
            green,
            blue,
            alpha: 255.0,
        }
    }

    pub fn pseudo_complement(&self) -> Rgb {
        Rgb {
            red: 255.0 - self.red,
            green: 255.0 - self.green,
            blue: 255.0 - self.blue,
            alpha: self.alpha,
        }
    }

    pub fn average(&self, other: Rgb) -> Rgb {
        Rgb {
            red: (self.red + other.red) / 2.0,
            green: (self.green + other.green) / 2.0,
            blue: (self.blue + other.blue) / 2.0,
            alpha: (self.alpha + other.alpha) / 2.0,
        }
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // todo: should these be double spaced..?
        write!(
            f,
            "rgb({}  {}  {} / {}%)",
            self.red,
            self.green,
            self.blue,
            ((self.alpha / 255.0) * 100.0).trunc()
        )
    }
}

make_range_checker!(HueChecker, 0.0, 360.0);
make_range_checker!(PercentChecker, 0.0, 100.0);

#[derive(Debug, Clone, Copy, ScamperStruct)]
pub struct Hsv {
    #[contract(HueChecker)]
    pub hue: f64,
    #[contract(PercentChecker)]
    pub saturation: f64,
    #[contract(PercentChecker)]
    pub value: f64,
    #[contract(RgbComponentChecker)]
    #[default(255.0)]
    pub alpha: f64,
}

impl Hsv {
    pub fn complement(&self) -> Self {
        Self {
            hue: (self.hue + 180.0) % 360.0,
            saturation: self.saturation,
            value: self.value,
            alpha: self.alpha,
        }
    }

    pub fn to_rgb(&self) -> Rgb {
        let hue = self.hue;
        let saturation = self.saturation / 100.0;
        let value = self.value / 100.0;

        let c = value * saturation;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = value - c;

        let (r, g, b) = if hue < 60.0 {
            (c, x, 0.0)
        } else if hue < 120.0 {
            (x, c, 0.0)
        } else if hue < 180.0 {
            (0.0, c, x)
        } else if hue < 240.0 {
            (0.0, x, c)
        } else if hue < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Rgb {
            red: ((r + m) * 255.0).trunc(),
            green: ((g + m) * 255.0).trunc(),
            blue: ((b + m) * 255.0).trunc(),
            alpha: self.alpha,
        }
    }
}

impl Display for Hsv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // todo: why is only one double spaced..?
        write!(
            f,
            "hsv({} {}%  {}% / {}%)",
            self.hue,
            self.saturation.trunc(),
            self.value.trunc(),
            (self.alpha / 255.0) * 100.0
        )
    }
}

#[derive(Debug, Clone)]
pub struct Color(Rgb);

impl Color {
    pub fn from_rgb(rgb: Rgb) -> Self {
        Self(rgb)
    }

    pub fn from_string(string: impl AsRef<str>) -> Option<Self> {
        Rgb::from_name(string).map(|v| Self(v))
    }

    pub fn to_rgb(&self) -> Rgb {
        self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromValue for Color {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Struct(_) => {
                if let Some(rgb) = Rgb::from_value(value) {
                    Some(Self(rgb))
                } else if let Some(hsv) = Hsv::from_value(value) {
                    Some(Self(hsv.to_rgb()))
                } else {
                    None
                }
            }
            Value::String(str) => {
                if let Some(rgb) = Rgb::from_name(str) {
                    Some(Self(rgb))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn name() -> &'static str {
        "color"
    }
}

fn is_rgb_component_prim(value: f64) -> bool {
    value >= 0.0 && value <= 255.0
}

#[function]
fn is_rgb_component(value: f64) -> bool {
    is_rgb_component_prim(value)
}

// legacy function
// #[function]
// fn color(args: &[Value]) -> Result<Value, RuntimeError> {
//     if args.len() != 4 {
//         return Err(RuntimeError::new(
//             format!(
//                 "wrong number of arguments to color provided. Expected 4, received {}.",
//                 args.len()
//             ),
//             None,
//         ));
//     }
//     rgb(args)
// }

#[function]
fn rgb_distance(rgb1: Rgb, rgb2: Rgb) -> f64 {
    ((rgb1.red - rgb2.red).powi(2)
        + (rgb1.green - rgb2.green).powi(2)
        + (rgb1.blue - rgb2.blue).powi(2))
    .sqrt()
}

#[function]
fn hsv_complement(hsv: Hsv) -> Hsv {
    hsv.complement()
}

fn rgb_hue_prim(rgb: Rgb) -> f64 {
    let (r, g, b) = (rgb.red, rgb.green, rgb.blue);
    let max = f64::max(f64::max(r, g), b);
    let min = f64::min(f64::min(r, g), b);

    if (max - min) == 0.0 {
        rand::random::<f64>() * 360.0
    } else {
        let h = if max == r {
            (g - b) / (max - min)
        } else if max == g {
            2.0 + (b - r) / (max - min)
        } else {
            4.0 + (r - g) / (max - min)
        };

        60.0 * (if h < 0.0 { h + 6.0 } else { h }).round()
    }
}

fn rgb_saturation_prim(rgb: Rgb) -> f64 {
    let max = f64::max(f64::max(rgb.red, rgb.green), rgb.blue);
    let min = f64::min(f64::min(rgb.red, rgb.green), rgb.blue);

    if max == 0.0 {
        0.0
    } else {
        100.0 * ((max - min) / max)
    }
}

fn rgb_value_prim(rgb: Rgb) -> f64 {
    100.0 * (rgb.red.max(rgb.green).max(rgb.blue) / 255.0).round()
}

#[function]
fn rgb_hue(rgb: Rgb) -> f64 {
    rgb_hue_prim(rgb)
}

#[function]
fn rgb_saturation(rgb: Rgb) -> f64 {
    rgb_saturation_prim(rgb)
}

#[function]
fn rgb_value(rgb: Rgb) -> f64 {
    rgb_value_prim(rgb)
}

#[function]
fn is_color_name(name: String) -> bool {
    Rgb::from_name(&name).is_some()
}

#[function]
fn all_color_names() -> List {
    Rgb::NAMES
        .iter()
        .map(|name| Value::String(name.to_string()))
        .collect::<Vec<_>>()
        .into()
}

#[function]
fn find_colors(name: String) -> List {
    Rgb::NAMES
        .iter()
        .filter(|n| n.contains(&name.to_lowercase()))
        .map(|name| Value::String(name.to_string()))
        .collect::<Vec<_>>()
        .into()
}

#[function]
fn rgb_to_string(rgb: Rgb) -> String {
    rgb.to_string()
}

#[function]
fn hsv_to_string(hsv: Hsv) -> String {
    hsv.to_string()
}

#[function]
fn rgb_to_hsv(rgb: Rgb) -> Hsv {
    let hue = rgb_hue_prim(rgb);
    let saturation = rgb_saturation_prim(rgb);
    let value = rgb_value_prim(rgb);

    Hsv {
        hue,
        saturation,
        value,
        alpha: rgb.alpha,
    }
}

#[function]
fn hsv_to_rgb(hsv: Hsv) -> Rgb {
    hsv.to_rgb()
}

#[function]
fn color_name_to_rgb(name: String) -> Result<Rgb, RuntimeError> {
    Rgb::from_name(&name).ok_or(RuntimeError::new(
        format!("Unknown color name: {}", name),
        None,
    ))
}

#[function]
fn rgb_darker(rgb: Rgb) -> Rgb {
    Rgb {
        red: f64::max(0.0, rgb.red - 16.0),
        green: f64::max(0.0, rgb.green - 16.0),
        blue: f64::max(0.0, rgb.blue - 16.0),
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_lighter(rgb: Rgb) -> Rgb {
    Rgb {
        red: f64::min(255.0, rgb.red + 16.0),
        green: f64::min(255.0, rgb.green + 16.0),
        blue: f64::min(255.0, rgb.blue + 16.0),
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_redder(rgb: Rgb) -> Rgb {
    Rgb {
        red: f64::min(255.0, rgb.red + 32.0),
        green: f64::max(0.0, rgb.green - 16.0),
        blue: f64::max(0.0, rgb.blue - 16.0),
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_greener(rgb: Rgb) -> Rgb {
    Rgb {
        red: f64::max(0.0, rgb.red - 16.0),
        green: f64::min(255.0, rgb.green + 32.0),
        blue: f64::max(0.0, rgb.blue - 16.0),
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_bluer(rgb: Rgb) -> Rgb {
    Rgb {
        red: f64::max(0.0, rgb.red - 16.0),
        green: f64::max(0.0, rgb.green - 16.0),
        blue: f64::min(255.0, rgb.blue + 32.0),
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_pseudo_complement(rgb: Rgb) -> Rgb {
    rgb.pseudo_complement()
}

#[function]
fn rgb_grayscale(rgb: Rgb) -> Rgb {
    let gray = (0.30 * rgb.red + 0.59 * rgb.green + 0.11 * rgb.blue) / 3.0;
    Rgb {
        red: gray,
        green: gray,
        blue: gray,
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_phaseshift(rgb: Rgb) -> Rgb {
    let shift = 128.0;
    Rgb {
        red: (rgb.red + shift) % 256.0,
        green: (rgb.green + shift) % 256.0,
        blue: (rgb.blue + shift) % 256.0,
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_rotate_components(rgb: Rgb) -> Rgb {
    Rgb {
        red: rgb.green,
        green: rgb.blue,
        blue: rgb.red,
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_thin(rgb: Rgb) -> Rgb {
    Rgb {
        red: rgb.red,
        green: rgb.green,
        blue: rgb.blue,
        alpha: f64::max(0.0, rgb.alpha - 32.0),
    }
}

#[function]
fn rgb_thicken(rgb: Rgb) -> Rgb {
    Rgb {
        red: rgb.red,
        green: rgb.green,
        blue: rgb.blue,
        alpha: f64::min(255.0, rgb.alpha + 32.0),
    }
}

#[function]
fn rgb_add(rgb: Rgb, rgb2: Rgb) -> Rgb {
    Rgb {
        red: f64::min(255.0, rgb.red + rgb2.red),
        green: f64::min(255.0, rgb.green + rgb2.green),
        blue: f64::min(255.0, rgb.blue + rgb2.blue),
        alpha: f64::min(255.0, rgb.alpha + rgb2.alpha),
    }
}

#[function]
fn rgb_subtract(rgb: Rgb, rgb2: Rgb) -> Rgb {
    Rgb {
        red: f64::max(0.0, rgb.red - rgb2.red),
        green: f64::max(0.0, rgb.green - rgb2.green),
        blue: f64::max(0.0, rgb.blue - rgb2.blue),
        alpha: f64::max(0.0, rgb.alpha - rgb2.alpha),
    }
}

#[function]
fn rgb_average(rgb: Rgb, rgb2: Rgb) -> Rgb {
    rgb.average(rgb2)
}
