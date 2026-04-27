use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
    str::FromStr,
};

use png::ColorType;

use crate::{errors::ParseHexError, pixel::Pixel};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Rgb {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);

    #[inline]
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        assert!(0.0 <= red && red <= 1.0, "Red value must be between 0.0 and 1.0.");
        assert!(0.0 <= green && green <= 1.0, "Green value must be between 0.0 and 1.0.");
        assert!(0.0 <= blue && blue <= 1.0, "Blue value must be between 0.0 and 1.0.");

        Self { red, green, blue }
    }
}

impl Pixel for Rgb {
    const CHANNELS: usize = 3;
    const PNG_COLOUR_TYPE: ColorType = ColorType::Rgb;

    type Bytes = [u8; 3];

    #[inline]
    fn to_bytes(&self) -> Self::Bytes {
        [
            (self.red.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.green.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.blue.clamp(0.0, 1.0) * 255.0).round() as u8,
        ]
    }

    #[inline]
    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self {
            red: bytes[0] as f32 / 255.0,
            green: bytes[1] as f32 / 255.0,
            blue: bytes[2] as f32 / 255.0,
        }
    }

    #[inline]
    fn to_u32(&self) -> u32 {
        let bytes = self.to_bytes();
        ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32)
    }

    #[inline]
    fn from_u32(value: u32) -> Self {
        let r = ((value >> 16) & 0xFF) as u8;
        let g = ((value >> 8) & 0xFF) as u8;
        let b = (value & 0xFF) as u8;
        Self::from_bytes([r, g, b])
    }

    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseHexError> {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)?;
                Ok(Self::from_bytes([r, g, b]))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16)?;
                let g = u8::from_str_radix(&hex[2..4], 16)?;
                let b = u8::from_str_radix(&hex[4..6], 16)?;
                Ok(Self::from_bytes([r, g, b]))
            }
            found => Err(ParseHexError::InvalidLength {
                expected: &[3, 6],
                found,
            }),
        }
    }
}

impl Add for Rgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl AddAssign for Rgb {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl Mul<f32> for Rgb {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul for Rgb {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl MulAssign<f32> for Rgb {
    fn mul_assign(&mut self, rhs: f32) {
        self.red *= rhs;
        self.green *= rhs;
        self.blue *= rhs;
    }
}

impl MulAssign for Rgb {
    fn mul_assign(&mut self, rhs: Self) {
        self.red *= rhs.red;
        self.green *= rhs.green;
        self.blue *= rhs.blue;
    }
}

impl Div<f32> for Rgb {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        }
    }
}

impl DivAssign<f32> for Rgb {
    fn div_assign(&mut self, rhs: f32) {
        self.red /= rhs;
        self.green /= rhs;
        self.blue /= rhs;
    }
}

impl Sum for Rgb {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::BLACK, |a, b| a + b)
    }
}

impl From<u32> for Rgb {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl From<Rgb> for u32 {
    fn from(value: Rgb) -> Self {
        value.to_u32()
    }
}

impl FromStr for Rgb {
    type Err = ParseHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.to_hex())
    }
}
