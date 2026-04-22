use crate::{
    colour::Rgb,
    shader::Block,
    tracing::{Hit, Probe},
};

pub trait Shader {
    fn emitted(&self, hit: &Hit) -> Rgb;
    fn reflected(&self, probe: &Probe, hit: &Hit) -> Rgb;
}

pub enum ShaderEnum {
    Block(Block),
}

impl Shader for ShaderEnum {
    fn emitted(&self, hit: &Hit) -> Rgb {
        match self {
            ShaderEnum::Block(block) => block.emitted(hit),
        }
    }

    fn reflected(&self, probe: &Probe, hit: &Hit) -> Rgb {
        match self {
            ShaderEnum::Block(block) => block.reflected(probe, hit),
        }
    }
}
