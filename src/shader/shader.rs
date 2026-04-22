use crate::{
    colour::Rgb,
    lighting::LightSample,
    shader::{Block, Lambertion},
    tracing::{Probe, WorldHit},
};

pub trait Shader {
    fn emitted(&self, hit: &WorldHit) -> Rgb;
    fn shade(&self, probe: &Probe, hit: &WorldHit, light: &LightSample) -> Rgb;
}

pub enum ShaderEnum {
    Block(Block),
    Lambertion(Lambertion),
}

impl Shader for ShaderEnum {
    fn emitted(&self, hit: &WorldHit) -> Rgb {
        match self {
            ShaderEnum::Block(block) => block.emitted(hit),
            ShaderEnum::Lambertion(lambertion) => lambertion.emitted(hit),
        }
    }

    fn shade(&self, probe: &Probe, hit: &WorldHit, light: &LightSample) -> Rgb {
        match self {
            ShaderEnum::Block(block) => block.shade(probe, hit, light),
            ShaderEnum::Lambertion(lambertion) => lambertion.shade(probe, hit, light),
        }
    }
}
