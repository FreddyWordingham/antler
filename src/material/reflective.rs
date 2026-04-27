use crate::{
    material::Material,
    tracing::{Probe, WorldHit, WorldRay},
};

#[derive(Debug, Clone, Copy)]
pub struct Reflective {
    pub reflectivity: f32,
}

impl Reflective {
    pub fn new(reflectivity: f32) -> Self {
        Self {
            reflectivity: reflectivity.clamp(0.0, 1.0),
        }
    }
}

impl Material for Reflective {
    fn scatter(&self, probe: &Probe, hit: &WorldHit, mut emit_child: impl FnMut(f32, WorldRay)) -> f32 {
        let incoming = probe.ray.direction.into_inner();
        let normal = hit.normal.into_inner();
        let reflected = incoming - 2.0 * incoming.dot(&normal) * normal;

        emit_child(
            self.reflectivity,
            WorldRay::from_offset(hit.position, hit.normal, nalgebra::Unit::new_normalize(reflected)),
        );

        1.0 - self.reflectivity
    }
}
