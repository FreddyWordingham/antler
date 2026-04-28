use antler_geometry::{Contact, Ray};
use rand::Rng;

use crate::{
    bsdf::Bsdf,
    utils::{cosine_weighted_hemisphere, offset_origin},
};

pub struct Lambertian {
    albedo: f32,
}

impl Lambertian {
    pub fn new(albedo: f32) -> Self {
        Self {
            albedo: albedo.clamp(0.0, 1.0),
        }
    }
}

impl Bsdf for Lambertian {
    fn scatter<R: Rng, F: FnMut(Ray, f32)>(
        &self,
        rng: &mut R,
        _ray: &Ray,
        contact: &Contact,
        mut emit_child: F,
    ) -> f32 {
        let direction = cosine_weighted_hemisphere(rng, contact.normal);

        emit_child(
            Ray {
                origin: offset_origin(contact.position, contact.normal, direction),
                direction,
            },
            self.albedo,
        );

        0.0
    }
}
