use std::ops::Deref;

use nalgebra::{Point3, Similarity3, Unit, Vector3};

use crate::{geometry::Ray, tracing::ObjectRay};

pub const ORIGIN_BIAS: f32 = 1.0e-6;

pub struct WorldRay(Ray);

impl WorldRay {
    #[inline]
    pub fn new(ray: Ray) -> Self {
        Self(ray)
    }

    #[inline]
    pub fn from_offset(origin: Point3<f32>, normal: Unit<Vector3<f32>>, direction: Unit<Vector3<f32>>) -> Self {
        let sign = if direction.dot(&normal) >= 0.0 { 1.0 } else { -1.0 };
        let biased_origin = origin + *normal * (ORIGIN_BIAS * sign);

        Self::new(crate::geometry::Ray {
            origin: biased_origin,
            direction,
        })
    }

    #[inline]
    pub fn to_object_space(&self, inv_transform: &Similarity3<f32>) -> ObjectRay {
        ObjectRay::new(self.0.transform(inv_transform))
    }
}

impl Deref for WorldRay {
    type Target = Ray;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
