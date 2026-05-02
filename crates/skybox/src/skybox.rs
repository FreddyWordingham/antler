use antler_colour::Rgb;
use nalgebra::{Unit, Vector3};

use crate::{constant::Constant, gradient::Gradient, sky::Sky};

pub enum Skybox {
    Constant(Constant),
    Gradient(Gradient),
}

impl Sky for Skybox {
    #[inline]
    fn sample(&self, direction: &Unit<Vector3<f32>>) -> Rgb {
        match self {
            Self::Constant(constant) => constant.sample(direction),
            Self::Gradient(gradient) => gradient.sample(direction),
        }
    }
}

impl From<Constant> for Skybox {
    #[inline]
    fn from(val: Constant) -> Self {
        Self::Constant(val)
    }
}

impl From<Gradient> for Skybox {
    #[inline]
    fn from(val: Gradient) -> Self {
        Self::Gradient(val)
    }
}
