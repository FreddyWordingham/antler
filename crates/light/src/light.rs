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
            Self::Directional(directional) => directional.for_each_sample(rng, contact, f),
        }
    }
}

impl From<Directional> for Light {
    #[inline]
    fn from(val: Directional) -> Self {
        Self::Directional(val)
    }
}
