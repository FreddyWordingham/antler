use crate::{
    geometry::{Aabb, Bounded, Sphere, Traceable},
    tracing::{ObjectHit, ObjectRay},
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
    fn trace(&self, ray: &ObjectRay) -> Option<ObjectHit> {
        match self {
            GeometryEnum::Aabb(aabb) => aabb.trace(ray),
            GeometryEnum::Sphere(sphere) => sphere.trace(ray),
        }
    }
}
