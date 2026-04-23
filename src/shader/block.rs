use crate::{
    colour::Rgb,
    lighting::LightSample,
    shader::Shader,
    tracing::{WorldHit, WorldRay},
};

pub struct Block {
    pub colour: Rgb,
}

impl Block {
    pub fn new(colour: Rgb) -> Self {
        Self { colour }
    }
}

impl Shader for Block {
    fn emitted(&self, _hit: &WorldHit) -> Rgb {
        Rgb::BLACK
    }

    fn shade(&self, _ray: &WorldRay, _hit: &WorldHit, _light: &LightSample) -> Rgb {
        self.colour
    }
}
