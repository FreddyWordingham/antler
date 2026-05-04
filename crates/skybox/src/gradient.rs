use antler_colour::{Rgb, RgbGradient};
use nalgebra::{Unit, Vector3};

use crate::sky::Sky;

pub struct Gradient {
    colours: RgbGradient,
    power: f32,
    up: Unit<Vector3<f32>>,
}

impl Gradient {
    #[must_use]
    pub const fn new(colours: RgbGradient, power: f32, up: Unit<Vector3<f32>>) -> Self {
        Self { colours, power, up }
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
        self.colours.sample(t)
    }
}
