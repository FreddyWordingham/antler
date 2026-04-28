use antler_geometry::{Intersection, Ray};

use crate::{bsdf::Bsdf, mirror::Mirror, opaque::Opaque, reflective::Reflective, refractive::Refractive};

pub enum Material {
    Mirror(Mirror),
    Opaque(Opaque),
    Reflective(Reflective),
    Refractive(Refractive),
}

impl Bsdf for Material {
    fn scatter<F: FnMut(Ray, f32)>(&self, ray: &Ray, intersection: &Intersection, emit_child: F) -> f32 {
        match self {
            Material::Mirror(inner) => inner.scatter(ray, intersection, emit_child),
            Material::Opaque(inner) => inner.scatter(ray, intersection, emit_child),
            Material::Reflective(inner) => inner.scatter(ray, intersection, emit_child),
            Material::Refractive(inner) => inner.scatter(ray, intersection, emit_child),
        }
    }
}
