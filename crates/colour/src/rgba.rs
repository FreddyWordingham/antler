use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
    str::FromStr,
};

use png::ColorType;

use crate::{errors::ParseHexError, pixel::Pixel, utils::parse_hex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Rgba {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    #[must_use]
    #[inline]
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        assert!(0.0 <= red && red <= 1.0, "Red value must be between 0.0 and 1.0.");
        assert!(0.0 <= green && green <= 1.0, "Green value must be between 0.0 and 1.0.");
        assert!(0.0 <= blue && blue <= 1.0, "Blue value must be between 0.0 and 1.0.");
        assert!(0.0 <= alpha && alpha <= 1.0, "Alpha value must be between 0.0 and 1.0.");

        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl Pixel for Rgba {
    const CHANNELS: usize = 4;
    const PNG_COLOUR_TYPE: ColorType = ColorType::Rgba;

    type Bytes = [u8; 4];

    #[inline]
    fn to_bytes(&self) -> Self::Bytes {
        [
            (self.red.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.green.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.blue.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.alpha.clamp(0.0, 1.0) * 255.0).round() as u8,
        ]
    }

    #[inline]
    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self {
            red: f32::from(bytes[0]) / 255.0,
            green: f32::from(bytes[1]) / 255.0,
            blue: f32::from(bytes[2]) / 255.0,
            alpha: f32::from(bytes[3]) / 255.0,
        }
    }

    #[inline]
    fn to_u32(&self) -> u32 {
        let bytes = self.to_bytes();
        (u32::from(bytes[0]) << 24) | (u32::from(bytes[1]) << 16) | (u32::from(bytes[2]) << 8) | u32::from(bytes[3])
    }

    #[inline]
    fn from_u32(value: u32) -> Self {
        let r = ((value >> 24) & 0xFF) as u8;
        let g = ((value >> 16) & 0xFF) as u8;
        let b = ((value >> 8) & 0xFF) as u8;
        let a = (value & 0xFF) as u8;
        Self::from_bytes([r, g, b, a])
    }

    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseHexError> {
        Ok(Self::from_bytes(parse_hex::<4>(hex)?))
    }
}

impl Add for Rgba {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
            alpha: self.alpha + rhs.alpha,
        }
    }
}

impl AddAssign for Rgba {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
        self.alpha += rhs.alpha;
    }
}

impl Mul<f32> for Rgba {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
            alpha: self.alpha * rhs,
        }
    }
}

impl Mul for Rgba {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
            alpha: self.alpha * rhs.alpha,
        }
    }
}

impl MulAssign<f32> for Rgba {
    fn mul_assign(&mut self, rhs: f32) {
        self.red *= rhs;
        self.green *= rhs;
        self.blue *= rhs;
        self.alpha *= rhs;
    }
}

impl MulAssign for Rgba {
    fn mul_assign(&mut self, rhs: Self) {
        self.red *= rhs.red;
        self.green *= rhs.green;
        self.blue *= rhs.blue;
        self.alpha *= rhs.alpha;
    }
}

impl Div<f32> for Rgba {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
            alpha: self.alpha / rhs,
        }
    }
}

impl DivAssign<f32> for Rgba {
    fn div_assign(&mut self, rhs: f32) {
        self.red /= rhs;
        self.green /= rhs;
        self.blue /= rhs;
        self.alpha /= rhs;
    }
}

impl Sum for Rgba {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::TRANSPARENT, |a, b| a + b)
    }
}

impl From<u32> for Rgba {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl From<Rgba> for u32 {
    fn from(value: Rgba) -> Self {
        value.to_u32()
    }
}

impl FromStr for Rgba {
    type Err = ParseHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.to_hex())
    }
}
