use crate::{
    colour::Rgb,
    tracing::{Hit, Probe},
};

pub trait Shader {
    fn emitted(&self, hit: &Hit) -> Rgb;
    fn reflected(&self, probe: &Probe, hit: &Hit) -> Rgb;
}

pub enum ShaderEnum {}

impl Shader for ShaderEnum {
    fn emitted(&self, _hit: &Hit) -> Rgb {
        match self {
            _ => todo!(),
        }
    }

    fn reflected(&self, _probe: &Probe, _hit: &Hit) -> Rgb {
        match self {
            _ => todo!(),
        }
    }
}
