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
