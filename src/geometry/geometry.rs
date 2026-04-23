use crate::{
    geometry::{Aabb, Bounded, Circle, Quad, Sphere, Traceable},
    tracing::{ObjectHit, ObjectRay},
};

pub trait Geometry: Bounded + Traceable {}
impl<T: Bounded + Traceable> Geometry for T {}

pub enum GeometryEnum {
    Aabb(Aabb),
    Sphere(Sphere),
    Circle(Circle),
    Quad(Quad),
}

impl Bounded for GeometryEnum {
    fn bounds(&self) -> Aabb {
        match self {
            GeometryEnum::Aabb(aabb) => aabb.bounds(),
            GeometryEnum::Sphere(sphere) => sphere.bounds(),
            GeometryEnum::Circle(circle) => circle.bounds(),
            GeometryEnum::Quad(quad) => quad.bounds(),
        }
    }
}

impl Traceable for GeometryEnum {
    fn trace(&self, ray: &ObjectRay) -> Option<ObjectHit> {
        match self {
            GeometryEnum::Aabb(aabb) => aabb.trace(ray),
            GeometryEnum::Sphere(sphere) => sphere.trace(ray),
            GeometryEnum::Circle(circle) => circle.trace(ray),
            GeometryEnum::Quad(quad) => quad.trace(ray),
        }
    }
}
