use antler_geometry::{Contact, Ray, utils::offset_origin};
use rand::Rng;

use crate::bsdf::Bsdf;

pub struct Transparent {
    pub transparency: f32,
}

impl Transparent {
    #[must_use]
    pub const fn new(transparency: f32) -> Self {
        Self {
            transparency: transparency.clamp(0.0, 1.0),
        }
    }
}

impl Bsdf for Transparent {
    fn scatter<R: Rng, F: FnMut(Ray, f32)>(
        &self,
        _rng: &mut R,
        ray: &Ray,
        contact: &Contact,
        mut emit_child: F,
    ) -> f32 {
        emit_child(
            Ray {
                origin: offset_origin(contact.position, -contact.normal, ray.direction),
                direction: ray.direction,
            },
            self.transparency,
        );

        1.0 - self.transparency
    }
}
