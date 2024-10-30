use crate::interpreter::Env;

mod color;
mod color_names;
mod drawing;

pub use color::{Color, Hsv, Rgb};
pub use drawing::{Align, Drawing, Mode};

pub fn add_to(env: &mut Env) {
    color::add_to(env);
    drawing::add_to(env);
}
