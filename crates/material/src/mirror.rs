use antler_geometry::{Intersection, Ray};

use crate::{bsdf::Bsdf, utils::reflect};

pub struct Mirror;

impl Mirror {
    pub fn new() -> Self {
        Self
    }
}

impl Bsdf for Mirror {
    fn scatter<F: FnMut(Ray, f32)>(&self, ray: &Ray, intersection: &Intersection, mut emit_child: F) -> f32 {
        emit_child(
            Ray {
                origin: intersection.position,
                direction: reflect(*ray.direction, *intersection.normal),
            },
            1.0,
        );

        0.0
    }
}
