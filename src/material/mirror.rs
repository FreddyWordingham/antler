use crate::{
    material::{Material, Scatter},
    tracing::{Probe, WorldHit, WorldRay},
};

pub struct Mirror;

impl Mirror {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Mirror {
    fn default() -> Self {
        Self::new()
    }
}

impl Material for Mirror {
    fn scatter(&self, probe: &Probe, hit: &WorldHit) -> Scatter {
        let incoming = probe.ray.direction.into_inner();
        let normal = hit.normal.into_inner();

        let reflected = incoming - 2.0 * incoming.dot(&normal) * normal;

        Scatter {
            local_fraction: 0.0,
            children: vec![(
                1.0,
                WorldRay::from_offset(hit.position, hit.normal, nalgebra::Unit::new_normalize(reflected)),
            )],
        }
    }
}
