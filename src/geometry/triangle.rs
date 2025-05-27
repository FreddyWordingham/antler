//! Smooth triangle structure.

use nalgebra::{Point3, RealField, Unit, Vector3};

use crate::geometry::{Aabb, Bounded, Intersection, Ray};

/// Three-dimensional triangle with interpolated surface normals.
pub struct Triangle<T: RealField> {
    /// Vertex positions.
    vertices: [Point3<T>; 3],
    /// Vertex normals.
    normals: [Unit<Vector3<T>>; 3],
}

impl<T: RealField> Triangle<T> {
    /// Construct a new `Triangle` instance.
    #[must_use]
    #[inline]
    pub const fn new(vertices: [Point3<T>; 3], normals: [Unit<Vector3<T>>; 3]) -> Self {
        Self { vertices, normals }
    }

    /// Test for an intersection between a `Ray` and the `Triangle`.
    pub fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>> {
        const EPSILON: f64 = 1e-8;
        let epsilon = T::from_f64(EPSILON)?;

        // Get triangle edges
        let edge1 = &self.vertices[1] - &self.vertices[0];
        let edge2 = &self.vertices[2] - &self.vertices[0];

        // Begin calculating determinant - also used to calculate u parameter
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        // If determinant is near zero, ray lies in plane of triangle
        if a > -epsilon.clone() && a < epsilon.clone() {
            return None;
        }

        let f = T::one() / a;
        let s = &ray.origin - &self.vertices[0];
        let u = f.clone() * s.dot(&h);

        // Check if intersection lies outside triangle
        if u < T::zero() || u > T::one() {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f.clone() * ray.direction.dot(&q);

        // Check if intersection lies outside triangle
        if v < T::zero() || u.clone() + v.clone() > T::one() {
            return None;
        }

        // Calculate t, ray intersects triangle
        let t = f.clone() * edge2.dot(&q);

        // Ray intersection behind the origin
        if t <= epsilon {
            return None;
        }

        // Calculate geometric normal (face normal)
        let geometric_normal = Unit::new_normalize(edge1.cross(&edge2));

        // Calculate interpolated normal using barycentric coordinates
        let w = T::one() - u.clone() - v.clone(); // barycentric coordinate for vertex 0
        let interpolated_normal =
            Unit::new_normalize(self.normals[0].scale(w) + self.normals[1].scale(u) + self.normals[2].scale(v));

        Some(Intersection::new(t, geometric_normal, interpolated_normal))
    }
}

impl<T: RealField> Bounded<T> for Triangle<T> {
    /// Get the `Aabb` bounding the `Triangle`.
    #[inline]
    fn aabb(&self) -> Aabb<T> {
        let mut mins = Point3::new(T::max_value().unwrap(), T::max_value().unwrap(), T::max_value().unwrap());
        let mut maxs = Point3::new(T::min_value().unwrap(), T::min_value().unwrap(), T::min_value().unwrap());

        for vertex in &self.vertices {
            mins = Point3::new(
                mins.x.clone().min(vertex.x.clone()),
                mins.y.clone().min(vertex.y.clone()),
                mins.z.clone().min(vertex.z.clone()),
            );
            maxs = Point3::new(
                maxs.x.clone().max(vertex.x.clone()),
                maxs.y.clone().max(vertex.y.clone()),
                maxs.z.clone().max(vertex.z.clone()),
            );
        }

        Aabb::new(mins, maxs)
    }
}
