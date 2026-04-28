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
            Geometry::Aabb(aabb) => aabb.bounds(),
            Geometry::Capsule(capsule) => capsule.bounds(),
            Geometry::Circle(circle) => circle.bounds(),
            Geometry::Mesh(mesh) => mesh.bounds(),
            Geometry::Quad(quad) => quad.bounds(),
            Geometry::Sphere(sphere) => sphere.bounds(),
            Geometry::Torus(torus) => torus.bounds(),
            Geometry::Triangle(triangle) => triangle.bounds(),
        }
    }
}

impl Traceable for Geometry {
    #[inline]
    fn hit(&self, ray: &Ray, max_distance: f32) -> bool {
        match self {
            Geometry::Aabb(aabb) => aabb.hit(ray, max_distance),
            Geometry::Capsule(capsule) => capsule.hit(ray, max_distance),
            Geometry::Circle(circle) => circle.hit(ray, max_distance),
            Geometry::Mesh(mesh) => mesh.hit(ray, max_distance),
            Geometry::Quad(quad) => quad.hit(ray, max_distance),
            Geometry::Sphere(sphere) => sphere.hit(ray, max_distance),
            Geometry::Torus(torus) => torus.hit(ray, max_distance),
            Geometry::Triangle(triangle) => triangle.hit(ray, max_distance),
        }
    }

    #[inline]
    fn distance(&self, ray: &Ray, max_distance: f32) -> Option<f32> {
        match self {
            Geometry::Aabb(aabb) => aabb.distance(ray, max_distance),
            Geometry::Capsule(capsule) => capsule.distance(ray, max_distance),
            Geometry::Circle(circle) => circle.distance(ray, max_distance),
            Geometry::Mesh(mesh) => mesh.distance(ray, max_distance),
            Geometry::Quad(quad) => quad.distance(ray, max_distance),
            Geometry::Sphere(sphere) => sphere.distance(ray, max_distance),
            Geometry::Torus(torus) => torus.distance(ray, max_distance),
            Geometry::Triangle(triangle) => triangle.distance(ray, max_distance),
        }
    }

    #[inline]
    fn intersection(&self, ray: &Ray, max_distance: f32) -> Option<Contact> {
        match self {
            Geometry::Aabb(aabb) => aabb.intersection(ray, max_distance),
            Geometry::Capsule(capsule) => capsule.intersection(ray, max_distance),
            Geometry::Circle(circle) => circle.intersection(ray, max_distance),
            Geometry::Mesh(mesh) => mesh.intersection(ray, max_distance),
            Geometry::Quad(quad) => quad.intersection(ray, max_distance),
            Geometry::Sphere(sphere) => sphere.intersection(ray, max_distance),
            Geometry::Torus(torus) => torus.intersection(ray, max_distance),
            Geometry::Triangle(triangle) => triangle.intersection(ray, max_distance),
        }
    }
}
