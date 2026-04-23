use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
};

use palette::Srgb;
use png::ColorType;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::{
    colour::{Pixel, Rgba},
    errors::ParseHexError,
};

#[derive(Debug, Clone, Copy)]
pub struct Rgb(Srgb);

impl Rgb {
    pub const BLACK: Rgb = Rgb(Srgb::new(0.0, 0.0, 0.0));

    #[inline]
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self(Srgb::new(red, green, blue))
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
        1.0
    }

    #[inline]
    pub fn to_rgba(&self) -> Rgba {
        Rgba::new(self.0.red, self.0.green, self.0.blue, 1.0)
    }
}

impl Add for Rgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Rgb {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Mul for Rgb {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<f32> for Rgb {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<f32> for Rgb {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
    }
}

impl Div<f32> for Rgb {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<f32> for Rgb {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
    }
}

impl Sum for Rgb {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::BLACK, |a, b| a + b)
    }
}

impl Pixel for Rgb {
    const CHANNELS: usize = 3;
    const PNG_COLOUR_TYPE: ColorType = ColorType::Rgb;

    type Bytes = [u8; 3];

    #[inline]
    fn to_bytes(&self) -> Self::Bytes {
        [
            (self.0.red.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.0.green.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.0.blue.clamp(0.0, 1.0) * 255.0).round() as u8,
        ]
    }

    #[inline]
    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self(Srgb::new(
            bytes[0] as f32 / 255.0,
            bytes[1] as f32 / 255.0,
            bytes[2] as f32 / 255.0,
        ))
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

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for Rgb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex = String::deserialize(deserializer)?;
        Rgb::from_hex(&hex).map_err(Error::custom)
    }
}
