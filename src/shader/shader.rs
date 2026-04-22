use crate::{
    colour::Rgb,
    shader::Block,
    tracing::{Probe, WorldHit},
};

pub trait Shader {
    fn emitted(&self, hit: &WorldHit) -> Rgb;
    fn reflected(&self, probe: &Probe, hit: &WorldHit) -> Rgb;
}

pub enum ShaderEnum {
    Block(Block),
}

impl Shader for ShaderEnum {
    fn emitted(&self, hit: &WorldHit) -> Rgb {
        match self {
            ShaderEnum::Block(block) => block.emitted(hit),
        }
    }

    fn reflected(&self, probe: &Probe, hit: &WorldHit) -> Rgb {
        match self {
            ShaderEnum::Block(block) => block.reflected(probe, hit),
        }
    }
}
