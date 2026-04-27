use crate::{
    colour::{Gradient, Rgb},
    lighting::LightSample,
    shader::Shader,
    tracing::{WorldHit, WorldRay},
};

pub struct GradientShader {
    pub gradient: Gradient<Rgb>,
}

impl GradientShader {
    pub fn new(gradient: Gradient<Rgb>) -> Self {
        Self { gradient }
    }
}

impl Shader for GradientShader {
    #[inline]
    fn emitted(&self, _hit: &WorldHit) -> Rgb {
        Rgb::BLACK
    }

    #[inline]
    fn albedo(&self, _hit: &WorldHit) -> Rgb {
        self.gradient.sample(1.0)
    }

    #[inline]
    fn shade(&self, hit: &WorldHit, _ray: &WorldRay, light: &LightSample) -> Rgb {
        let t = hit.normal.dot(&light.direction).max(0.0);
        self.gradient.sample(t) * light.radiance
    }
}
