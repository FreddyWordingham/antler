//! Engine sampling function.

use crate::{
    input::Shader,
    output::Sample,
    parts::Scene,
    parts::{Attributes, Tracer},
};
use arctk::{geom::Hit, math::Dir3, phys::Crossing};
use palette::{Gradient, LinSrgba};
use rand::rngs::ThreadRng;
use std::fmt::Display;

/// Sample the scene using the tracer.
#[inline]
#[must_use]
pub fn paint<T: Display + Ord>(
    mut rng: &mut ThreadRng,
    scene: &Scene<T>,
    shader: &Shader,
    mut trace: Tracer,
) -> Sample {
    Sample::new(LinSrgba::default())
}
