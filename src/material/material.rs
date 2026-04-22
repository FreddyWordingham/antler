use crate::{
    material::{Opaque, Scatter},
    tracing::{Hit, Probe},
};

pub trait Material {
    fn scatter(&self, probe: &Probe, hit: &Hit) -> Scatter;
}

pub enum MaterialEnum {
    Opaque(Opaque),
}

impl Material for MaterialEnum {
    fn scatter(&self, probe: &Probe, hit: &Hit) -> Scatter {
        match self {
            MaterialEnum::Opaque(opaque) => opaque.scatter(probe, hit),
        }
    }
}
