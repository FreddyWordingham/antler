use crate::{
    colour::Rgb,
    lighting::LightSample,
    shader::Shader,
    tracing::{Probe, WorldHit},
};

pub struct Block {
    pub colour: Rgb,
}

impl Shader for Block {
    fn emitted(&self, _hit: &WorldHit) -> Rgb {
        Rgb::BLACK
    }

    fn shade(&self, _probe: &Probe, _hit: &WorldHit, _light: &LightSample) -> Rgb {
        self.colour
    }
}
