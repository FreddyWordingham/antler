use crate::{
    lighting::{DirectionalLight, LightSample},
    tracing::WorldHit,
};

pub trait Light {
    fn sample(&self, hit: &WorldHit) -> LightSample;
}

pub enum LightEnum {
    DirectionalLight(DirectionalLight),
}

impl Light for LightEnum {
    fn sample(&self, hit: &WorldHit) -> LightSample {
        match self {
            LightEnum::DirectionalLight(light) => light.sample(hit),
        }
    }
}
