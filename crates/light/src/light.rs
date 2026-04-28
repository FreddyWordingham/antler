use rand::Rng;

use crate::{directional::Directional, emissive::Emissive};

pub enum Light {
    Directional(Directional),
}

impl Emissive for Light {
    fn for_each_sample<R: Rng, F: FnMut(crate::light_sample::LightSample)>(
        &self,
        rng: &mut R,
        contact: &antler_geometry::Contact,
        f: F,
    ) {
        match self {
            Light::Directional(directional) => directional.for_each_sample(rng, contact, f),
        }
    }
}
