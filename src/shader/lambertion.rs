use crate::{
    colour::Rgb,
    lighting::LightSample,
    shader::Shader,
    tracing::{WorldHit, WorldRay},
};

pub struct Lambertion {
    pub colour: Rgb,
}

impl Lambertion {
    pub fn new(colour: Rgb) -> Self {
        Self { colour }
    }
}

impl Shader for Lambertion {
    fn emitted(&self, _hit: &WorldHit) -> Rgb {
        Rgb::BLACK
    }

    fn shade(&self, _ray: &WorldRay, hit: &WorldHit, light: &LightSample) -> Rgb {
        let n_dot_l = hit.normal.dot(&light.direction).max(0.0);
        self.colour * light.radiance * n_dot_l
    }
}
