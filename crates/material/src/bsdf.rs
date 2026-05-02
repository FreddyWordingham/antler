use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use rand::Rng;

pub trait Bsdf {
    #[must_use]
    #[inline]
    fn visibility(&self) -> Rgb {
        Rgb::BLACK
    }

    fn scatter<R: Rng, F: FnMut(Ray, f32)>(&self, rng: &mut R, ray: &Ray, contact: &Contact, emit_child: F) -> f32;
}
