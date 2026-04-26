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
    #[inline]
    fn emitted(&self, _hit: &WorldHit) -> Rgb {
        Rgb::BLACK
    }

    #[inline]
    fn albedo(&self, _hit: &WorldHit) -> Rgb {
        self.colour
    }

    #[inline]
    fn shade(&self, hit: &WorldHit, _ray: &WorldRay, light: &LightSample) -> Rgb {
        let n_dot_l = hit.normal.dot(&light.direction).max(0.0);
        self.albedo(hit) * light.radiance * n_dot_l
    }
}
