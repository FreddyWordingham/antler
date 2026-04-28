use antler_geometry::{Contact, Ray};
use rand::Rng;

use crate::bsdf::Bsdf;

pub struct Opaque;

impl Opaque {
    pub fn new() -> Self {
        Self
    }
}

impl Bsdf for Opaque {
    fn scatter<R: Rng, F: FnMut(Ray, f32)>(&self, _rng: &mut R, _ray: &Ray, _contact: &Contact, _emit_child: F) -> f32 {
        1.0
    }
}
