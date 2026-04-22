use nalgebra::{Unit, Vector3};

use crate::colour::Rgb;

pub struct LightSample {
    pub direction: Unit<Vector3<f32>>,
    pub distance: f32,
    pub radiance: Rgb,
}
