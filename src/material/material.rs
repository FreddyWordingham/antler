use crate::{
    material::{Opaque, Scatter},
    tracing::{Probe, WorldHit},
};

pub trait Material {
    fn scatter(&self, probe: &Probe, hit: &WorldHit) -> Scatter;
}

pub enum MaterialEnum {
    Opaque(Opaque),
}

impl Material for MaterialEnum {
    fn scatter(&self, probe: &Probe, hit: &WorldHit) -> Scatter {
        match self {
            MaterialEnum::Opaque(opaque) => opaque.scatter(probe, hit),
        }
    }
}
