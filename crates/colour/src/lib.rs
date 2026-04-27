pub mod errors;
mod pixel;
mod rgb;
mod rgba;
mod utils;

pub mod prelude {
    pub use crate::{errors::ParseHexError, pixel::Pixel, rgb::Rgb, rgba::Rgba};
}
