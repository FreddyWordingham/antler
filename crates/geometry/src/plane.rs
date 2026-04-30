use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{config::MIN_RAY_DISTANCE, ray::Ray};

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub position: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,
    pub tangent: Unit<Vector3<f32>>,
    pub bi_tangent: Unit<Vector3<f32>>,
}

impl Plane {
    #[must_use]
    pub fn new(position: Point3<f32>, normal: Unit<Vector3<f32>>) -> Self {
        let n = normal.into_inner();

        let helper_axis = if n.x.abs() < 0.9 {
            Vector3::x_axis().into_inner()
        } else {
            Vector3::y_axis().into_inner()
        };

        let tangent = Unit::new_normalize(helper_axis.cross(&n));
        let bi_tangent = Unit::new_normalize(n.cross(&tangent));

        Self {
            position,
            normal,
            tangent,
            bi_tangent,
        }
    }

    #[inline]
    pub fn ray_distance(&self, ray: &Ray) -> Option<f32> {
        let denom = self.normal.dot(&ray.direction);

        if denom.abs() <= MIN_RAY_DISTANCE {
            return None;
        }

        let distance = (self.position - ray.origin).dot(&self.normal) / denom;

        (distance > MIN_RAY_DISTANCE.max(MIN_RAY_DISTANCE)).then_some(distance)
    }

    #[inline]
    pub fn project(&self, point: Point3<f32>) -> Point2<f32> {
        let local = point - self.position;

        Point2::new(local.dot(&self.tangent), local.dot(&self.bi_tangent))
    }

    #[inline]
    pub fn normal_for_ray(&self, ray: &Ray) -> Unit<Vector3<f32>> {
        if self.normal.dot(&ray.direction) > 0.0 {
            -self.normal
        } else {
            self.normal
        }
    }
}
