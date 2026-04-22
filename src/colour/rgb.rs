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
