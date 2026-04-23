use std::f32::INFINITY;

use nalgebra::{Unit, Vector3};

use crate::{
    colour::Rgb,
    lighting::{Light, LightSample},
    tracing::WorldHit,
};

pub struct DirectionalLight {
    pub direction: Unit<Vector3<f32>>,
    pub radiance: Rgb,
}

impl DirectionalLight {
    pub fn new(direction: Unit<Vector3<f32>>, radiance: Rgb) -> Self {
        Self { direction, radiance }
    }
}

impl Light for DirectionalLight {
    fn sample(&self, _hit: &WorldHit) -> LightSample {
        LightSample {
            direction: -self.direction,
            distance: INFINITY,
            radiance: self.radiance,
        }
    }
}
