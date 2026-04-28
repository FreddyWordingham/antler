use nalgebra::{Point3, Similarity3, Unit, Vector3};

pub const ORIGIN_BIAS: f32 = 1e-4;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Unit<Vector3<f32>>,
}

impl Ray {
    #[must_use]
    #[inline]
    pub const fn new(origin: Point3<f32>, direction: Unit<Vector3<f32>>) -> Self {
        Self { origin, direction }
    }

    #[must_use]
    #[inline]
    pub fn from_offset(origin: Point3<f32>, normal: Unit<Vector3<f32>>, direction: Unit<Vector3<f32>>) -> Self {
        let scale = origin.coords.abs().max();
        let bias = ORIGIN_BIAS * scale.max(1.0);

        let sign = if direction.dot(&normal) >= 0.0 { 1.0 } else { -1.0 };
        let biased_origin = origin + *normal * (bias * sign);

        Self::new(biased_origin, direction)
    }

    #[must_use]
    #[inline]
    pub fn transform(&self, transform: &Similarity3<f32>) -> Self {
        Self {
            origin: transform.transform_point(&self.origin),
            direction: Unit::new_normalize(transform.transform_vector(&self.direction)),
        }
    }
}
