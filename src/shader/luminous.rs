use crate::{
    colour::Rgb,
    lighting::LightSample,
    shader::Shader,
    tracing::{WorldHit, WorldRay},
};

pub struct Luminous {
    pub colour: Rgb,
    pub intensity: f32,
}

impl Luminous {
    pub fn new(colour: Rgb, intensity: f32) -> Self {
        Self { colour, intensity }
    }
}

impl Shader for Luminous {
    #[inline]
    fn emitted(&self, _hit: &WorldHit) -> Rgb {
        self.colour * self.intensity
    }

    #[inline]
    fn shade(&self, _ray: &WorldRay, _hit: &WorldHit, _light: &LightSample) -> Rgb {
        Rgb::BLACK
    }
}
