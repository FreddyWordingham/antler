use crate::{
    aabb::Aabb, bounded::Bounded, capsule::Capsule, circle::Circle, contact::Contact, mesh::Mesh, quad::Quad, ray::Ray,
    sphere::Sphere, torus::Torus, traceable::Traceable, triangle::Triangle,
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
