mod errors;
mod pixel;
mod rgb;
mod rgba;

pub mod prelude {
    pub use crate::{errors::ParseHexError, pixel::Pixel, rgb::Rgb, rgba::Rgba};
}
