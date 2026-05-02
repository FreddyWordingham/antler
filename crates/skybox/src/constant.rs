use antler_colour::Rgb;
use nalgebra::{Unit, Vector3};

use crate::sky::Sky;

pub struct Constant {
    colour: Rgb,
}

impl Constant {
    pub const fn new(colour: Rgb) -> Self {
        Self { colour }
    }
}

impl Sky for Constant {
    #[inline]
    fn sample(&self, _direction: &Unit<Vector3<f32>>) -> Rgb {
        self.colour
    }
}
