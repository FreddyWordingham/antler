use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign},
};

use palette::Srgba;

pub struct Rgba(Srgba);

impl Rgba {
    pub const BLACK: Rgba = Rgba(Srgba::new(0.0, 0.0, 0.0, 1.0));
    pub const TRANSPARENT: Rgba = Rgba(Srgba::new(0.0, 0.0, 0.0, 0.0));

    #[inline]
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        assert!(red >= 0.0 && red <= 1.0);
        assert!(green >= 0.0 && green <= 1.0);
        assert!(blue >= 0.0 && blue <= 1.0);
        assert!(alpha >= 0.0 && alpha <= 1.0);
        Self(Srgba::new(red, green, blue, alpha))
    }

    #[inline]
    pub const fn from_bytes(bytes: [u8; 4]) -> Self {
        Self(Srgba::new(
            bytes[0] as f32 / 255.0,
            bytes[1] as f32 / 255.0,
            bytes[2] as f32 / 255.0,
            bytes[3] as f32 / 255.0,
        ))
    }

    #[inline]
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        match hex.len() {
            4 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).expect("Invalid hex string");
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).expect("Invalid hex string");
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).expect("Invalid hex string");
                let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).expect("Invalid hex string");
                Self::from_bytes([r, g, b, a])
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid hex string");
                let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid hex string");
                let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid hex string");
                let a = u8::from_str_radix(&hex[6..8], 16).expect("Invalid hex string");
                Self::from_bytes([r, g, b, a])
            }
            _ => panic!("Hex string must be 4 or 8 characters long"),
        }
    }

    #[inline]
    pub fn to_bytes(&self) -> [u8; 4] {
        [
            (self.0.red * 255.0).round() as u8,
            (self.0.green * 255.0).round() as u8,
            (self.0.blue * 255.0).round() as u8,
            (self.0.alpha * 255.0).round() as u8,
        ]
    }

    #[inline]
    pub fn to_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            (self.0.red * 255.0).round() as u8,
            (self.0.green * 255.0).round() as u8,
            (self.0.blue * 255.0).round() as u8,
            (self.0.alpha * 255.0).round() as u8
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

    #[inline]
    pub fn alpha(&self) -> f32 {
        self.0.alpha
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

impl Sum for Rgba {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::BLACK, |a, b| a + b)
    }
}
