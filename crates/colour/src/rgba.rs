use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
};

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
        iter.fold(Self::BLACK, |a, b| a + b)
    }
}
