use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
    str::FromStr,
};

use png::ColorType;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::{
    errors::ParseHexError,
    pixel::Pixel,
    rgba::Rgba,
    utils::{parse_hex, reinhard},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Rgb {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);

    #[must_use]
    #[inline]
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    #[must_use]
    #[inline]
    pub const fn to_rgba(&self) -> crate::Rgba {
        Rgba::new(self.red, self.green, self.blue, 1.0)
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
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            red: f32::from(bytes[0]) / 255.0,
            green: f32::from(bytes[1]) / 255.0,
            blue: f32::from(bytes[2]) / 255.0,
        }
    }

    #[inline]
    fn to_u32(&self) -> u32 {
        let bytes = self.to_bytes();
        (u32::from(bytes[0]) << 16) | (u32::from(bytes[1]) << 8) | u32::from(bytes[2])
    }

    #[inline]
    fn from_u32(value: u32) -> Self {
        let r = ((value >> 16) & 0xFF) as u8;
        let g = ((value >> 8) & 0xFF) as u8;
        let b = (value & 0xFF) as u8;
        Self::from_bytes(&[r, g, b])
    }

    #[inline]
    fn from_hex(hex: &str) -> Result<Self, ParseHexError> {
        Ok(Self::from_bytes(&parse_hex::<3>(hex)?))
    }

    #[inline]
    fn tone_mapped(&self) -> Self {
        Self {
            red: reinhard(self.red),
            green: reinhard(self.green),
            blue: reinhard(self.blue),
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
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum RgbRepr {
            Int(u32),
            Hex(String),
        }

        match RgbRepr::deserialize(deserializer)? {
            RgbRepr::Int(value) => Ok(Rgb::from_u32(value)),
            RgbRepr::Hex(hex) => Rgb::from_hex(&hex).map_err(Error::custom),
        }
    }
}
