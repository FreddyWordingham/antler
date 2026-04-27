use antler_geometry::{Intersection, Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, intersection: &Intersection, emit_child: impl FnMut(Ray, f32)) -> f32;
}
