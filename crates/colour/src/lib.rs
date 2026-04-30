pub mod errors;
mod gradient;
mod pixel;
mod rgb;
mod rgba;
mod utils;

pub use crate::{
    gradient::{Gradient, RgbGradient, RgbaGradient},
    pixel::Pixel,
    rgb::Rgb,
    rgba::Rgba,
};
