use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use rand::Rng;

use crate::{
    bsdf::Bsdf, ggx::Ggx, lambertian::Lambertian, mirror::Mirror, opaque::Opaque, reflective::Reflective,
    refractive::Refractive, transparent::Transparent, wireframe::Wireframe,
};

pub enum Material {
    Ggx(Ggx),
    Lambertian(Lambertian),
    Mirror(Mirror),
    Opaque(Opaque),
    Reflective(Reflective),
    Refractive(Refractive),
    Transparent(Transparent),
    Wireframe(Wireframe),
}

impl Bsdf for Material {
    fn visibility(&self) -> Rgb {
        match self {
            Self::Ggx(inner) => inner.visibility(),
            Self::Lambertian(inner) => inner.visibility(),
            Self::Mirror(inner) => inner.visibility(),
            Self::Opaque(inner) => inner.visibility(),
            Self::Reflective(inner) => inner.visibility(),
            Self::Refractive(inner) => inner.visibility(),
            Self::Transparent(inner) => inner.visibility(),
            Self::Wireframe(inner) => inner.visibility(),
        }
    }

    fn scatter<R: Rng, F: FnMut(Ray, f32)>(&self, rng: &mut R, ray: &Ray, contact: &Contact, emit_child: F) -> f32 {
        match self {
            Self::Ggx(inner) => inner.scatter(rng, ray, contact, emit_child),
            Self::Lambertian(inner) => inner.scatter(rng, ray, contact, emit_child),
            Self::Mirror(inner) => inner.scatter(rng, ray, contact, emit_child),
            Self::Opaque(inner) => inner.scatter(rng, ray, contact, emit_child),
            Self::Reflective(inner) => inner.scatter(rng, ray, contact, emit_child),
            Self::Refractive(inner) => inner.scatter(rng, ray, contact, emit_child),
            Self::Transparent(inner) => inner.scatter(rng, ray, contact, emit_child),
            Self::Wireframe(inner) => inner.scatter(rng, ray, contact, emit_child),
        }
    }
}

impl From<Ggx> for Material {
    #[inline]
    fn from(val: Ggx) -> Self {
        Self::Ggx(val)
    }
}

impl From<Lambertian> for Material {
    #[inline]
    fn from(val: Lambertian) -> Self {
        Self::Lambertian(val)
    }
}

impl From<Mirror> for Material {
    #[inline]
    fn from(val: Mirror) -> Self {
        Self::Mirror(val)
    }
}

impl From<Opaque> for Material {
    #[inline]
    fn from(val: Opaque) -> Self {
        Self::Opaque(val)
    }
}

impl From<Reflective> for Material {
    #[inline]
    fn from(val: Reflective) -> Self {
        Self::Reflective(val)
    }
}

impl From<Refractive> for Material {
    #[inline]
    fn from(val: Refractive) -> Self {
        Self::Refractive(val)
    }
}

impl From<Transparent> for Material {
    #[inline]
    fn from(val: Transparent) -> Self {
        Self::Transparent(val)
    }
}

impl From<Wireframe> for Material {
    #[inline]
    fn from(val: Wireframe) -> Self {
        Self::Wireframe(val)
    }
}
