use antler_geometry::{Contact, Ray, utils::offset_origin};
use rand::Rng;

use crate::{bsdf::Bsdf, utils::reflect};

pub struct Mirror;

impl Default for Mirror {
    fn default() -> Self {
        Self::new()
    }
}

impl Mirror {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Bsdf for Mirror {
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
            1.0,
        );

        0.0
    }
}
