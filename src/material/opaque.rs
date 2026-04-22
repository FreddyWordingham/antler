use crate::{
    material::{Material, Scatter},
    tracing::{Hit, Probe},
};

pub struct Opaque;

impl Material for Opaque {
    fn scatter(&self, _probe: &Probe, _hit: &Hit) -> Scatter {
        Scatter {
            absorbed: 1.0,
            children: Vec::new(),
        }
    }
}
