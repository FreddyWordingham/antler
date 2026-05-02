use rand::Rng;

use crate::sample::Sample;

pub trait Sampleable {
    fn area(&self) -> f32;

    fn sample<R: Rng>(&self, rng: &mut R) -> Sample;
}
