use nalgebra::{Point2, Point3, Unit, Vector2, Vector3};

use crate::{aabb::Aabb, bounded::Bounded, contact::Contact, plane::Plane, ray::Ray, traceable::Traceable};

pub struct Quad {
    plane: Plane,
    size: Vector2<f32>,
}

impl Quad {
    #[must_use]
    pub fn new(centre: Point3<f32>, normal: Unit<Vector3<f32>>, size: Vector2<f32>) -> Self {
        assert!(size.x > 0.0, "Quad width must be positive");
        assert!(size.y > 0.0, "Quad height must be positive");

        Self {
            plane: Plane::new(centre, normal),
            size,
        }
    }

    #[inline]
    fn half_size(&self) -> Vector2<f32> {
        self.size / 2.0
    }

    #[inline]
    fn local_hit(&self, ray: &Ray, max_distance: f32) -> Option<(f32, Point2<f32>)> {
        let distance = self.plane.ray_distance(ray)?;

        if distance >= max_distance {
            return None;
        }

        let position = ray.origin + *ray.direction * distance;
        let local = self.plane.project(position);
        let half = self.half_size();

        (local.x.abs() <= half.x && local.y.abs() <= half.y).then_some((distance, local))
    }
}

impl Bounded for Quad {
    fn bounds(&self) -> Aabb {
        let half = self.half_size();

        let ex = self.plane.tangent.into_inner() * half.x;
        let ey = self.plane.bi_tangent.into_inner() * half.y;

        let corners = [
            self.plane.position + ex + ey,
            self.plane.position + ex - ey,
            self.plane.position - ex + ey,
            self.plane.position - ex - ey,
        ];

        Aabb::new(
            Point3::new(
                corners.iter().map(|p| p.x).fold(f32::INFINITY, f32::min),
                corners.iter().map(|p| p.y).fold(f32::INFINITY, f32::min),
                corners.iter().map(|p| p.z).fold(f32::INFINITY, f32::min),
            ),
            Point3::new(
                corners.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max),
                corners.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max),
                corners.iter().map(|p| p.z).fold(f32::NEG_INFINITY, f32::max),
            ),
        )
    }
}

impl Traceable for Quad {
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

        let uv = Point2::new(0.5 + local.x / self.size.x, 0.5 + local.y / self.size.y);

        Some(Contact::new(distance, position, self.plane.normal_for_ray(ray), uv))
    }
}
