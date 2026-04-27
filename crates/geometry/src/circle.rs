use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{aabb::Aabb, bounded::Bounded, intersection::Intersection, plane::Plane, ray::Ray, traceable::Traceable};

pub struct Circle {
    frame: Plane,
    radius: f32,
}

impl Circle {
    #[must_use]
    pub fn new(position: Point3<f32>, normal: Unit<Vector3<f32>>, radius: f32) -> Self {
        assert!(radius > 0.0, "Circle radius must be positive");

        Self {
            frame: Plane::new(position, normal),
            radius,
        }
    }

    #[inline]
    fn local_hit(&self, ray: &Ray, max_distance: f32) -> Option<(f32, Point2<f32>)> {
        let distance = self.frame.ray_distance(ray)?;

        if distance >= max_distance {
            return None;
        }

        let position = ray.origin + *ray.direction * distance;
        let local = self.frame.project(position);

        (local.coords.norm_squared() <= self.radius * self.radius).then_some((distance, local))
    }
}

impl Bounded for Circle {
    fn bounds(&self) -> Aabb {
        let n = self.frame.normal.into_inner().abs();

        let extent = Vector3::new(
            self.radius * (1.0 - n.x * n.x).sqrt(),
            self.radius * (1.0 - n.y * n.y).sqrt(),
            self.radius * (1.0 - n.z * n.z).sqrt(),
        );

        Aabb::new(self.frame.position - extent, self.frame.position + extent)
    }
}

impl Traceable for Circle {
    #[inline]
    fn hit(&self, ray: &Ray, max_distance: f32) -> bool {
        self.local_hit(ray, max_distance).is_some()
    }

    #[inline]
    fn distance(&self, ray: &Ray, max_distance: f32) -> Option<f32> {
        self.local_hit(ray, max_distance).map(|(distance, _)| distance)
    }

    #[inline]
    fn intersection(&self, ray: &Ray, max_distance: f32) -> Option<Intersection> {
        let (distance, local) = self.local_hit(ray, max_distance)?;
        let position = ray.origin + *ray.direction * distance;

        let uv = Point2::new(0.5 + local.x / (2.0 * self.radius), 0.5 + local.y / (2.0 * self.radius));

        Some(Intersection::new(
            distance,
            position,
            self.frame.normal_for_ray(ray),
            uv,
        ))
    }
}
