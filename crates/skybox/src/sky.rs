use antler_colour::Rgb;
use nalgebra::{Unit, Vector3};

pub trait Sky {
    fn sample(&self, direction: &Unit<Vector3<f32>>) -> Rgb;
}
