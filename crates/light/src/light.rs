use rand::Rng;

use crate::{directional::Directional, emissive::Emissive, environment::Environment, point::Point};

pub enum Light {
    Directional(Directional),
    Environment(Environment),
    Point(Point),
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
            Self::Environment(environment) => environment.for_each_sample(rng, contact, f),
            Self::Point(point) => point.for_each_sample(rng, contact, f),
        }
    }
}

impl From<Directional> for Light {
    #[inline]
    fn from(val: Directional) -> Self {
        Self::Directional(val)
    }
}

impl From<Environment> for Light {
    #[inline]
    fn from(val: Environment) -> Self {
        Self::Environment(val)
    }
}

impl From<Point> for Light {
    #[inline]
    fn from(val: Point) -> Self {
        Self::Point(val)
    }
}
