use crate::{
    colour::Rgb,
    tracing::{Hit, Photon},
};

pub trait Shader {
    fn emitted(&self, hit: &Hit) -> Rgb;
    fn reflected(&self, photon: &Photon, hit: &Hit) -> Rgb;
}

pub enum ShaderEnum {}

impl Shader for ShaderEnum {
    fn emitted(&self, _hit: &Hit) -> Rgb {
        match self {
            _ => todo!(),
        }
    }

    fn reflected(&self, _photon: &Photon, _hit: &Hit) -> Rgb {
        match self {
            _ => todo!(),
        }
    }
}
