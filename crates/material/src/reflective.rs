use antler_geometry::{Contact, Ray, utils::offset_origin};
use rand::Rng;

use crate::{bsdf::Bsdf, utils::reflect};

pub struct Reflective {
    reflectance: f32,
}

impl Reflective {
    #[must_use]
    pub const fn new(reflectance: f32) -> Self {
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
        contact: &Contact,
        mut emit_child: F,
    ) -> f32 {
        let direction = reflect(ray.direction, contact.normal);

        emit_child(
            Ray {
                origin: offset_origin(contact.position, contact.normal, direction),
                direction,
            },
            self.reflectance,
        );

        1.0 - self.reflectance
    }
}
