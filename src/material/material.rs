use crate::{
    material::Scatter,
    tracing::{Hit, Probe},
};

pub trait Material {
    fn scatter(&self, probe: &Probe, hit: &Hit) -> Scatter;
}

pub enum MaterialEnum {}

impl Material for MaterialEnum {
    fn scatter(&self, _probe: &Probe, _hit: &Hit) -> Scatter {
        match self {
            _ => todo!(),
        }
    }
}
