use antler_geometry::{Intersection, Ray};

use crate::material::Material;

pub struct Opaque;

impl Opaque {
    pub fn new() -> Self {
        Self
    }
}

impl Material for Opaque {
    fn scatter(&self, _ray: &Ray, _intersection: &Intersection, _emit_child: impl FnMut(Ray, f32)) -> f32 {
        1.0
    }
}
