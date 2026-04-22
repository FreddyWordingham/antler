use crate::{
    material::{Material, Scatter},
    tracing::{Probe, WorldHit},
};

pub struct Opaque;

impl Material for Opaque {
    fn scatter(&self, _probe: &Probe, _hit: &WorldHit) -> Scatter {
        Scatter {
            local_fraction: 1.0,
            children: Vec::new(),
        }
    }
}
