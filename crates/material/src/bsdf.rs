use antler_geometry::{Intersection, Ray};

pub trait Bsdf {
    fn scatter<F: FnMut(Ray, f32)>(&self, ray: &Ray, intersection: &Intersection, emit_child: F) -> f32;
}
