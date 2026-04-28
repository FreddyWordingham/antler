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
            Self::Ggx(inner) => inner.scatter(rng, ray, contact, emit_child),
            Self::Lambertian(inner) => inner.scatter(rng, ray, contact, emit_child),
            Self::Mirror(inner) => inner.scatter(rng, ray, contact, emit_child),
            Self::Opaque(inner) => inner.scatter(rng, ray, contact, emit_child),
            Self::Reflective(inner) => inner.scatter(rng, ray, contact, emit_child),
            Self::Refractive(inner) => inner.scatter(rng, ray, contact, emit_child),
        }
    }
}

impl Into<Material> for Ggx {
    #[inline]
    fn into(self) -> Material {
        Material::Ggx(self)
    }
}

impl Into<Material> for Lambertian {
    #[inline]
    fn into(self) -> Material {
        Material::Lambertian(self)
    }
}

impl Into<Material> for Mirror {
    #[inline]
    fn into(self) -> Material {
        Material::Mirror(self)
    }
}

impl Into<Material> for Opaque {
    #[inline]
    fn into(self) -> Material {
        Material::Opaque(self)
    }
}

impl Into<Material> for Reflective {
    #[inline]
    fn into(self) -> Material {
        Material::Reflective(self)
    }
}

impl Into<Material> for Refractive {
    #[inline]
    fn into(self) -> Material {
        Material::Refractive(self)
    }
}
