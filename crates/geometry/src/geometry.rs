use rand::Rng;

use crate::{
    aabb::Aabb, bounded::Bounded, capsule::Capsule, circle::Circle, contact::Contact, mesh::Mesh, quad::Quad, ray::Ray,
    sample::Sample, sampleable::Sampleable, sphere::Sphere, torus::Torus, traceable::Traceable, triangle::Triangle,
};

pub enum Geometry {
    Aabb(Aabb),
    Capsule(Capsule),
    Circle(Circle),
    Mesh(Mesh),
    Quad(Quad),
    Sphere(Sphere),
    Torus(Torus),
    Triangle(Triangle),
}

impl Bounded for Geometry {
    #[inline]
    fn bounds(&self) -> Aabb {
        match self {
            Self::Aabb(aabb) => aabb.bounds(),
            Self::Capsule(capsule) => capsule.bounds(),
            Self::Circle(circle) => circle.bounds(),
            Self::Mesh(mesh) => mesh.bounds(),
            Self::Quad(quad) => quad.bounds(),
            Self::Sphere(sphere) => sphere.bounds(),
            Self::Torus(torus) => torus.bounds(),
            Self::Triangle(triangle) => triangle.bounds(),
        }
    }
}

impl Traceable for Geometry {
    #[inline]
    fn hit(&self, ray: &Ray, max_distance: f32) -> bool {
        match self {
            Self::Aabb(aabb) => aabb.hit(ray, max_distance),
            Self::Capsule(capsule) => capsule.hit(ray, max_distance),
            Self::Circle(circle) => circle.hit(ray, max_distance),
            Self::Mesh(mesh) => mesh.hit(ray, max_distance),
            Self::Quad(quad) => quad.hit(ray, max_distance),
            Self::Sphere(sphere) => sphere.hit(ray, max_distance),
            Self::Torus(torus) => torus.hit(ray, max_distance),
            Self::Triangle(triangle) => triangle.hit(ray, max_distance),
        }
    }

    #[inline]
    fn distance(&self, ray: &Ray, max_distance: f32) -> Option<f32> {
        match self {
            Self::Aabb(aabb) => aabb.distance(ray, max_distance),
            Self::Capsule(capsule) => capsule.distance(ray, max_distance),
            Self::Circle(circle) => circle.distance(ray, max_distance),
            Self::Mesh(mesh) => mesh.distance(ray, max_distance),
            Self::Quad(quad) => quad.distance(ray, max_distance),
            Self::Sphere(sphere) => sphere.distance(ray, max_distance),
            Self::Torus(torus) => torus.distance(ray, max_distance),
            Self::Triangle(triangle) => triangle.distance(ray, max_distance),
        }
    }

    #[inline]
    fn intersection(&self, ray: &Ray, max_distance: f32) -> Option<Contact> {
        match self {
            Self::Aabb(aabb) => aabb.intersection(ray, max_distance),
            Self::Capsule(capsule) => capsule.intersection(ray, max_distance),
            Self::Circle(circle) => circle.intersection(ray, max_distance),
            Self::Mesh(mesh) => mesh.intersection(ray, max_distance),
            Self::Quad(quad) => quad.intersection(ray, max_distance),
            Self::Sphere(sphere) => sphere.intersection(ray, max_distance),
            Self::Torus(torus) => torus.intersection(ray, max_distance),
            Self::Triangle(triangle) => triangle.intersection(ray, max_distance),
        }
    }
}

impl Sampleable for Geometry {
    #[inline]
    fn area(&self) -> f32 {
        match self {
            Self::Quad(quad) => quad.area(),
            _ => unimplemented!("Area not implemented for this geometry type"),
        }
    }

    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> Sample {
        match self {
            Self::Quad(quad) => quad.sample(rng),
            _ => unimplemented!("Sampling not implemented for this geometry type"),
        }
    }
}

impl From<Aabb> for Geometry {
    #[inline]
    fn from(val: Aabb) -> Self {
        Self::Aabb(val)
    }
}

impl From<Capsule> for Geometry {
    #[inline]
    fn from(val: Capsule) -> Self {
        Self::Capsule(val)
    }
}

impl From<Circle> for Geometry {
    #[inline]
    fn from(val: Circle) -> Self {
        Self::Circle(val)
    }
}

impl From<Mesh> for Geometry {
    #[inline]
    fn from(val: Mesh) -> Self {
        Self::Mesh(val)
    }
}

impl From<Quad> for Geometry {
    #[inline]
    fn from(val: Quad) -> Self {
        Self::Quad(val)
    }
}

impl From<Sphere> for Geometry {
    #[inline]
    fn from(val: Sphere) -> Self {
        Self::Sphere(val)
    }
}

impl From<Torus> for Geometry {
    #[inline]
    fn from(val: Torus) -> Self {
        Self::Torus(val)
    }
}

impl From<Triangle> for Geometry {
    #[inline]
    fn from(val: Triangle) -> Self {
        Self::Triangle(val)
    }
}
