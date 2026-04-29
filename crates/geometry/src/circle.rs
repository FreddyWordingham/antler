use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{aabb::Aabb, bounded::Bounded, contact::Contact, plane::Plane, ray::Ray, traceable::Traceable};

pub struct Circle {
    plane: Plane,
    radius: f32,
}

impl Circle {
    #[must_use]
    pub fn new(centre: Point3<f32>, normal: Unit<Vector3<f32>>, radius: f32) -> Self {
        assert!(radius > 0.0, "Circle radius must be positive");

        Self {
            plane: Plane::new(centre, normal),
            radius,
        }
    }

    #[inline]
    fn local_hit(&self, ray: &Ray, max_distance: f32) -> Option<(f32, Point2<f32>)> {
        let distance = self.plane.ray_distance(ray)?;

        if distance >= max_distance {
            return None;
        }

        let position = ray.origin + *ray.direction * distance;
        let local = self.plane.project(position);

        (local.coords.norm_squared() <= self.radius * self.radius).then_some((distance, local))
    }
}

impl Bounded for Circle {
    fn bounds(&self) -> Aabb {
        let n = self.plane.normal.into_inner().abs();

        let extent = Vector3::new(
            self.radius * n.x.mul_add(-n.x, 1.0).sqrt(),
            self.radius * n.y.mul_add(-n.y, 1.0).sqrt(),
            self.radius * n.z.mul_add(-n.z, 1.0).sqrt(),
        );

        Aabb::new(self.plane.position - extent, self.plane.position + extent)
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
    fn intersection(&self, ray: &Ray, max_distance: f32) -> Option<Contact> {
        let (distance, local) = self.local_hit(ray, max_distance)?;
        let position = ray.origin + *ray.direction * distance;

        let uv = Point2::new(0.5 + local.x / (2.0 * self.radius), 0.5 + local.y / (2.0 * self.radius));

        Some(Contact::new(distance, position, self.plane.normal_for_ray(ray), uv))
    }
}
