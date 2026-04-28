use antler_geometry::{Intersection, Ray};
use rand::Rng;

pub trait Bsdf {
    fn scatter<R: Rng, F: FnMut(Ray, f32)>(
        &self,
        rng: &mut R,
        ray: &Ray,
        intersection: &Intersection,
        emit_child: F,
    ) -> f32;
}
