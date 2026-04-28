use antler_geometry::{Contact, Ray};
use rand::Rng;

use crate::{
    bsdf::Bsdf, ggx::Ggx, lambertian::Lambertian, mirror::Mirror, opaque::Opaque, reflective::Reflective,
    refractive::Refractive,
};

pub enum Material {
    Ggx(Ggx),
    Lambertian(Lambertian),
    Mirror(Mirror),
    Opaque(Opaque),
    Reflective(Reflective),
    Refractive(Refractive),
}

impl Bsdf for Material {
    fn scatter<R: Rng, F: FnMut(Ray, f32)>(&self, rng: &mut R, ray: &Ray, contact: &Contact, emit_child: F) -> f32 {
        match self {
            Material::Ggx(inner) => inner.scatter(rng, ray, contact, emit_child),
            Material::Lambertian(inner) => inner.scatter(rng, ray, contact, emit_child),
            Material::Mirror(inner) => inner.scatter(rng, ray, contact, emit_child),
            Material::Opaque(inner) => inner.scatter(rng, ray, contact, emit_child),
            Material::Reflective(inner) => inner.scatter(rng, ray, contact, emit_child),
            Material::Refractive(inner) => inner.scatter(rng, ray, contact, emit_child),
        }
    }
}
