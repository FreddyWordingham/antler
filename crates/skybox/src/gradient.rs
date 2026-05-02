use antler_colour::{Rgb, RgbGradient};
use nalgebra::{Unit, Vector3};

use crate::sky::Sky;

pub struct Gradient {
    gradient: RgbGradient,
    power: f32,
    up: Unit<Vector3<f32>>,
}

impl Gradient {
    pub fn new(gradient: RgbGradient, power: f32, up: Unit<Vector3<f32>>) -> Self {
        Self { gradient, power, up }
    }
}

impl Sky for Gradient {
    #[inline]
    fn sample(&self, direction: &Unit<Vector3<f32>>) -> Rgb {
        let t = direction
            .dot(&self.up)
            .mul_add(0.5, 0.5)
            .clamp(0.0, 1.0)
            .powf(self.power);
        self.gradient.sample(t)
    }
}
