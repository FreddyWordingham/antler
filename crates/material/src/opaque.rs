use antler_geometry::{Intersection, Ray};

use crate::bsdf::Bsdf;

pub struct Opaque;

impl Opaque {
    pub fn new() -> Self {
        Self
    }
}

impl Bsdf for Opaque {
    fn scatter<F: FnMut(Ray, f32)>(&self, _ray: &Ray, _intersection: &Intersection, _emit_child: F) -> f32 {
        1.0
    }
}
