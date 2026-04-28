use antler_geometry::{Intersection, Ray};
use rand::Rng;

use crate::{bsdf::Bsdf, utils::reflect};

pub struct Mirror;

impl Mirror {
    pub fn new() -> Self {
        Self
    }
}

impl Bsdf for Mirror {
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
            1.0,
        );

        0.0
    }
}
