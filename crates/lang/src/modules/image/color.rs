use scamper_macros::{function, ForeignValue};

use crate::{
    ast::Value,
    interpreter::{Env, RuntimeError},
};

pub fn add_to(env: &mut Env) {
    env.register("rgb", rgb);
    env.register("color", color);
    env.register("rgb-red", rgb_red);
    env.register("rgb-green", rgb_green);
    env.register("rgb-blue", rgb_blue);
    env.register("rgb-alpha", rgb_alpha);
    env.register("rgb-darker", rgb_darker);
    env.register("rgb-lighter", rgb_lighter);
    env.register("rgb-redder", rgb_redder);
    env.register("rgb-greener", rgb_greener);
    env.register("rgb-bluer", rgb_bluer);
    env.register("rgb-pseudo-complement", rgb_pseudo_complement);
}

#[derive(Debug, Clone, ForeignValue)]
pub struct Rgb {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
    pub alpha: i64,
}

impl Rgb {
    pub fn pseudo_complement(&self) -> Rgb {
        Rgb {
            red: 255 - self.red,
            green: 255 - self.green,
            blue: 255 - self.blue,
            alpha: self.alpha,
        }
    }
}

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "rgb({} {} {} / {}%)",
            self.red,
            self.green,
            self.blue,
            ((self.alpha as f64 / 255.0) * 100.0) as i64
        )
    }
}

// TODO: support optionals
#[function]
fn rgb(args: &[i64]) -> Result<Rgb, RuntimeError> {
    if args.len() != 3 && args.len() != 4 {
        return Err(RuntimeError::new(
            format!("rgb: expects 3 or 4 arguments, but got {}", args.len()),
            None,
        ));
    }
    for arg in args {
        if *arg < 0 || *arg > 255 {
            return Err(RuntimeError::new(
                "expected a number in the range 0–255".to_string(),
                None,
            ));
        }
    }
    Ok(Rgb {
        red: args[0],
        green: args[1],
        blue: args[2],
        alpha: args.get(3).copied().unwrap_or(255),
    })
}

// legacy function
#[function]
fn color(args: &[Value]) -> Result<Value, RuntimeError> {
    if args.len() != 4 {
        return Err(RuntimeError::new(
            format!(
                "wrong number of arguments to color provided. Expected 4, received {}.",
                args.len()
            ),
            None,
        ));
    }
    rgb(args)
}

#[function]
fn rgb_red(rgb: Rgb) -> i64 {
    rgb.red
}

#[function]
fn rgb_green(rgb: Rgb) -> i64 {
    rgb.green
}

#[function]
fn rgb_blue(rgb: Rgb) -> i64 {
    rgb.blue
}

#[function]
fn rgb_alpha(rgb: Rgb) -> i64 {
    rgb.alpha
}

// transformations

#[function]
fn rgb_darker(rgb: Rgb) -> Rgb {
    Rgb {
        red: i64::max(0, rgb.red - 16),
        green: i64::max(0, rgb.green - 16),
        blue: i64::max(0, rgb.blue - 16),
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_lighter(rgb: Rgb) -> Rgb {
    Rgb {
        red: i64::min(255, rgb.red + 16),
        green: i64::min(255, rgb.green + 16),
        blue: i64::min(255, rgb.blue + 16),
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_redder(rgb: Rgb) -> Rgb {
    Rgb {
        red: i64::min(255, rgb.red + 32),
        green: i64::max(0, rgb.green - 16),
        blue: i64::max(0, rgb.blue - 16),
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_greener(rgb: Rgb) -> Rgb {
    Rgb {
        red: i64::max(0, rgb.red - 16),
        green: i64::min(255, rgb.green + 32),
        blue: i64::max(0, rgb.blue - 16),
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_bluer(rgb: Rgb) -> Rgb {
    Rgb {
        red: i64::max(0, rgb.red - 16),
        green: i64::max(0, rgb.green - 16),
        blue: i64::min(255, rgb.blue + 32),
        alpha: rgb.alpha,
    }
}

#[function]
fn rgb_pseudo_complement(rgb: Rgb) -> Rgb {
    rgb.pseudo_complement()
}
