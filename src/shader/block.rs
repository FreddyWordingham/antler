use crate::{
    colour::Rgb,
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

    fn reflected(&self, _probe: &Probe, _hit: &WorldHit) -> Rgb {
        self.colour
    }
}
