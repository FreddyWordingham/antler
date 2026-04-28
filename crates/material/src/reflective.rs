use antler_geometry::{Intersection, Ray};
use rand::Rng;

use crate::{bsdf::Bsdf, utils::reflect};

pub struct Reflective {
    reflectance: f32,
}

impl Reflective {
    pub fn new(reflectance: f32) -> Self {
        Self {
            reflectance: reflectance.clamp(0.0, 1.0),
        }
    }
}

impl Bsdf for Reflective {
    fn scatter<R: Rng, F: FnMut(Ray, f32)>(
        &self,
        _rng: &mut R,
        ray: &Ray,
        intersection: &Intersection,
        mut emit_child: F,
    ) -> f32 {
        emit_child(
            Ray {
                origin: intersection.position,
                direction: reflect(ray.direction, intersection.normal),
            },
            self.reflectance,
        );

        1.0 - self.reflectance
    }
}
