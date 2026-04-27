use crate::{
    material::Material,
    tracing::{Probe, WorldHit, WorldRay},
};

pub struct Opaque;

impl Opaque {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Opaque {
    fn default() -> Self {
        Self::new()
    }
}

impl Material for Opaque {
    fn scatter(&self, _probe: &Probe, _hit: &WorldHit, _emit_child: impl FnMut(f32, WorldRay)) -> f32 {
        1.0
    }
}
