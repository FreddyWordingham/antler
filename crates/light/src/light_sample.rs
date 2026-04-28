use antler_colour::Rgb;
use nalgebra::{Unit, Vector3};

pub struct LightSample {
    pub direction: Unit<Vector3<f32>>,
    pub distance: f32,
    pub radiance: Rgb,
}
