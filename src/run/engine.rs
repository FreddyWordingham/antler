//! Engine sampling function.

use crate::{input::Shader, output::Sample, parts::Scene, parts::Tracer};
use palette::LinSrgba;
use rand::rngs::ThreadRng;
use std::fmt::Display;

/// Sample the scene using the tracer.
#[inline]
#[must_use]
pub fn paint<T: Display + Ord>(
    mut _rng: &mut ThreadRng,
    _scene: &Scene<T>,
    _shader: &Shader,
    mut _trace: Tracer,
) -> Sample {
    Sample::new(LinSrgba::default())
}
