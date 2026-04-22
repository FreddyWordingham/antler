use crate::{
    colour::Rgb,
    shader::Shader,
    tracing::{Hit, Probe},
};

pub struct Block {
    pub colour: Rgb,
}

impl Shader for Block {
    fn emitted(&self, _hit: &Hit) -> Rgb {
        Rgb::BLACK
    }

    fn reflected(&self, _probe: &Probe, _hit: &Hit) -> Rgb {
        self.colour
    }
}
