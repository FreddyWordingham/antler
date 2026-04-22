use crate::{
    material::Scatter,
    tracing::{Hit, Photon},
};

pub trait Material {
    fn scatter(&self, photon: &Photon, hit: &Hit) -> Scatter;
}

pub enum MaterialEnum {}

impl MaterialEnum {
    pub fn scatter(&self, _photon: &Photon, _hit: &Hit) -> Scatter {
        match self {
            _ => todo!(),
        }
    }
}
