use nalgebra::Unit;

use crate::{
    material::{Material, Scatter},
    tracing::{Probe, WorldHit, WorldRay},
    utils::physics,
};

#[derive(Debug, Clone, Copy)]
pub struct Refractive {
    pub refractive_index: f32,
}

impl Refractive {
    pub fn new(refractive_index: f32) -> Self {
        assert!(refractive_index > 0.0);
        Self { refractive_index }
    }
}

impl Material for Refractive {
    fn scatter(&self, probe: &Probe, hit: &WorldHit) -> Scatter {
        let incoming = probe.ray.direction.into_inner();
        let mut normal = hit.normal.into_inner();

        let entering = incoming.dot(&normal) < 0.0;

        let eta = if entering {
            1.0 / self.refractive_index
        } else {
            normal = -normal;
            self.refractive_index
        };

        let cos_theta = (-incoming).dot(&normal).min(1.0);
        let sin2_theta = 1.0 - cos_theta * cos_theta;
        let cannot_refract = eta * eta * sin2_theta > 1.0;

        let direction = if cannot_refract {
            physics::reflect(incoming, normal)
        } else {
            physics::refract(incoming, normal, eta, cos_theta)
        };

        Scatter {
            local_fraction: 0.0,
            children: vec![(
                1.0,
                WorldRay::from_offset(
                    hit.position,
                    Unit::new_normalize(normal),
                    Unit::new_normalize(direction),
                ),
            )],
        }
    }
}
