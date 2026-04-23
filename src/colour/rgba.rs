use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
    str::FromStr,
};

use palette::Srgba;
use png::ColorType;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::{
    colour::{Pixel, Rgb},
    errors::ParseHexError,
};

#[derive(Debug, Clone, Copy)]
pub struct Rgba(Srgba);

impl Rgba {
    pub const BLACK: Rgba = Rgba(Srgba::new(0.0, 0.0, 0.0, 1.0));
    pub const TRANSPARENT: Rgba = Rgba(Srgba::new(0.0, 0.0, 0.0, 0.0));

    #[inline]
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self(Srgba::new(red, green, blue, alpha))
    }

    #[inline]
    pub fn red(&self) -> f32 {
        self.0.red
    }

    #[inline]
    pub fn green(&self) -> f32 {
        self.0.green
    }

    #[inline]
    pub fn blue(&self) -> f32 {
        self.0.blue
    }

    #[inline]
    pub fn alpha(&self) -> f32 {
        self.0.alpha
    }

    #[inline]
    pub fn to_rgb(&self) -> Rgb {
        Rgb::new(self.0.red, self.0.green, self.0.blue)
    }
}

impl Add for Rgba {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Rgba {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Mul for Rgba {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<f32> for Rgba {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<f32> for Rgba {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl Div<f32> for Rgba {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<f32> for Rgba {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
    }
}

impl Sum for Rgba {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::BLACK, |a, b| a + b)
    }
}

impl Pixel for Rgba {
    const CHANNELS: usize = 4;
    const PNG_COLOUR_TYPE: ColorType = ColorType::Rgba;

    type Bytes = [u8; 4];

    #[inline]
    fn to_bytes(&self) -> Self::Bytes {
        [
            (self.0.red.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.0.green.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.0.blue.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.0.alpha.clamp(0.0, 1.0) * 255.0).round() as u8,
        ]
    }

    #[inline]
    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self(Srgba::new(
            bytes[0] as f32 / 255.0,
            bytes[1] as f32 / 255.0,
            bytes[2] as f32 / 255.0,
            bytes[3] as f32 / 255.0,
        ))
    }

    #[inline]
    fn to_u32(&self) -> u32 {
        let bytes = self.to_bytes();
        ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | (bytes[3] as u32)
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
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            4 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)?;
                let a = u8::from_str_radix(&hex[3..4].repeat(2), 16)?;
                Ok(Self::from_bytes([r, g, b, a]))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16)?;
                let g = u8::from_str_radix(&hex[2..4], 16)?;
                let b = u8::from_str_radix(&hex[4..6], 16)?;
                let a = u8::from_str_radix(&hex[6..8], 16)?;
                Ok(Self::from_bytes([r, g, b, a]))
            }
            found => Err(ParseHexError::InvalidLength {
                expected: &[4, 8],
                found,
            }),
        }
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

impl Serialize for Rgba {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.to_u32())
    }
}

impl<'de> Deserialize<'de> for Rgba {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum RgbaRepr {
            Int(u32),
            Hex(String),
        }

        match RgbaRepr::deserialize(deserializer)? {
            RgbaRepr::Int(value) => Ok(Rgba::from_u32(value)),
            RgbaRepr::Hex(hex) => Rgba::from_hex(&hex).map_err(Error::custom),
        }
    }
}
