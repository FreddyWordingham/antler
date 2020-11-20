//! Colour particle.

use crate::{access, clone, geom::Ray};

/// Colour particle.
#[derive(Clone)]
pub struct Tracer {
    /// Ray of travel.
    ray: Ray,
    /// Statistical weighting.
    weight: f64,
}

impl Tracer {
    access!(ray, ray_mut, Ray);
    clone!(weight, weight_mut, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray, init_weight: f64) -> Self {
        debug_assert!(init_weight > 0.0);
        debug_assert!(init_weight <= 1.0);

        Self {
            ray,
            weight: init_weight,
        }
    }
}
