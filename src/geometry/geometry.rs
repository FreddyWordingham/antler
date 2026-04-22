use crate::{
    geometry::{Aabb, Bounded, Ray, Sphere, Traceable},
    tracing::Hit,
};

pub trait Geometry: Bounded + Traceable {}
impl<T: Bounded + Traceable> Geometry for T {}

pub enum GeometryEnum {
    Aabb(Aabb),
    Sphere(Sphere),
}

impl Bounded for GeometryEnum {
    fn bounds(&self) -> Aabb {
        match self {
            GeometryEnum::Aabb(aabb) => aabb.bounds(),
            GeometryEnum::Sphere(sphere) => sphere.bounds(),
        }
    }
}

impl Traceable for GeometryEnum {
    fn trace(&self, ray: &Ray) -> Option<Hit> {
        match self {
            GeometryEnum::Aabb(aabb) => aabb.trace(ray),
            GeometryEnum::Sphere(sphere) => sphere.trace(ray),
        }
    }
}
