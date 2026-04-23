use crate::{
    colour::Rgb,
    lighting::LightSample,
    shader::{Block, Lambertion, Luminous},
    tracing::{WorldHit, WorldRay},
};

pub trait Shader {
    fn emitted(&self, hit: &WorldHit) -> Rgb;
    fn shade(&self, ray: &WorldRay, hit: &WorldHit, light: &LightSample) -> Rgb;
}

pub enum ShaderEnum {
    Block(Block),
    Lambertion(Lambertion),
    Luminous(Luminous),
}

impl Shader for ShaderEnum {
    fn emitted(&self, hit: &WorldHit) -> Rgb {
        match self {
            ShaderEnum::Block(block) => block.emitted(hit),
            ShaderEnum::Lambertion(lambertion) => lambertion.emitted(hit),
            ShaderEnum::Luminous(luminous) => luminous.emitted(hit),
        }
    }

    fn shade(&self, ray: &WorldRay, hit: &WorldHit, light: &LightSample) -> Rgb {
        match self {
            ShaderEnum::Block(block) => block.shade(ray, hit, light),
            ShaderEnum::Lambertion(lambertion) => lambertion.shade(ray, hit, light),
            ShaderEnum::Luminous(luminous) => luminous.shade(ray, hit, light),
        }
    }
}
