use antler_geometry::{Intersection, Ray};
use rand::Rng;

use crate::{
    bsdf::Bsdf,
    utils::{offset_origin, reflect},
};

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
        let direction = reflect(ray.direction, intersection.normal);

        emit_child(
            Ray {
                origin: offset_origin(intersection.position, intersection.normal, direction),
                direction,
            },
            1.0,
        );

        0.0
    }
}
