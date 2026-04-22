use crate::{
    colour::Rgb,
    tracing::{Hit, Photon},
};

pub trait Shader {
    fn shade(&self, photon: &Photon, hit: &Hit) -> Rgb;
}

pub enum ShaderEnum {}

impl ShaderEnum {
    pub fn shade(&self, _photon: &Photon, _hit: &Hit) -> Rgb {
        match self {
            _ => todo!(),
        }
    }
}
