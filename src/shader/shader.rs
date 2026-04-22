use crate::{
    colour::Rgb,
    tracing::{Hit, Photon},
};

pub trait Shader {
    fn shade(&self, photon: &Photon, hit: &Hit) -> Rgb;
}

pub enum ShaderEnum {}
