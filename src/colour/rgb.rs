use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign},
};

use palette::Srgb;

pub struct Rgb(Srgb);

impl Rgb {
    pub const BLACK: Rgb = Rgb(Srgb::new(0.0, 0.0, 0.0));

    #[inline]
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        assert!(red >= 0.0 && red <= 1.0);
        assert!(green >= 0.0 && green <= 1.0);
        assert!(blue >= 0.0 && blue <= 1.0);
        Self(Srgb::new(red, green, blue))
    }

    #[inline]
    pub const fn from_bytes(bytes: [u8; 3]) -> Self {
        Self(Srgb::new(
            bytes[0] as f32 / 255.0,
            bytes[1] as f32 / 255.0,
            bytes[2] as f32 / 255.0,
        ))
    }

    #[inline]
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).expect("Invalid hex string");
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).expect("Invalid hex string");
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).expect("Invalid hex string");
                Self::from_bytes([r, g, b])
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid hex string");
                let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid hex string");
                let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid hex string");
                Self::from_bytes([r, g, b])
            }
            _ => panic!("Hex string must be 3 or 6 characters long"),
        }
    }

    #[inline]
    pub const fn to_bytes(&self) -> [u8; 3] {
        [
            (self.0.red * 255.0).round() as u8,
            (self.0.green * 255.0).round() as u8,
            (self.0.blue * 255.0).round() as u8,
        ]
    }

    #[inline]
    pub fn to_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            (self.0.red * 255.0).round() as u8,
            (self.0.green * 255.0).round() as u8,
            (self.0.blue * 255.0).round() as u8
        )
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

impl Sum for Rgb {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::BLACK, |a, b| a + b)
    }
}
