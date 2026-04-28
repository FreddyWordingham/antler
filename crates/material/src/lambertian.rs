use antler_geometry::{Intersection, Ray};
use rand::Rng;

use crate::{bsdf::Bsdf, utils::cosine_weighted_hemisphere};

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
        intersection: &Intersection,
        mut emit_child: F,
    ) -> f32 {
        emit_child(
            Ray {
                origin: intersection.position,
                direction: cosine_weighted_hemisphere(rng, intersection.normal),
            },
            self.albedo,
        );

        0.0
    }
}
