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
    #[inline]
    fn emitted(&self, _hit: &WorldHit) -> Rgb {
        Rgb::BLACK
    }

    #[inline]
    fn albedo(&self, _hit: &WorldHit) -> Rgb {
        self.colour
    }

    #[inline]
    fn shade(&self, hit: &WorldHit, _ray: &WorldRay, _light: &LightSample) -> Rgb {
        self.albedo(hit)
    }
}
