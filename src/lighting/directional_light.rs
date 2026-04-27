use std::f32::INFINITY;

use nalgebra::{Unit, Vector3};
use rand::Rng;

use crate::{
    colour::Rgb,
    lighting::{Light, LightSample},
    tracing::WorldHit,
    utils::sampling::cone_direction,
};

pub struct DirectionalLight {
    pub direction: Unit<Vector3<f32>>,
    pub radiance: Rgb,
    angular_radius: Option<f32>,
    samples: Option<usize>,
}

impl DirectionalLight {
    pub fn new(
        direction: Unit<Vector3<f32>>,
        radiance: Rgb,
        angular_radius: Option<f32>,
        samples: Option<usize>,
    ) -> Self {
        Self {
            direction,
            radiance,
            angular_radius,
            samples,
        }
    }
}

impl Light for DirectionalLight {
    fn for_each_sample(&self, _hit: &WorldHit, rng: &mut impl Rng, mut f: impl FnMut(LightSample)) {
        let angular_radius = self.angular_radius.unwrap_or(0.0);
        let samples = self.samples.unwrap_or(1).max(1);

        if angular_radius <= 0.0 || samples == 1 {
            f(LightSample {
                direction: -self.direction,
                distance: INFINITY,
                radiance: self.radiance,
            });
            return;
        }

        for _ in 0..samples {
            f(LightSample {
                direction: cone_direction(-self.direction, angular_radius, rng),
                distance: INFINITY,
                radiance: self.radiance,
            });
        }
    }
}
