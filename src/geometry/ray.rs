use nalgebra::{Point3, Similarity3, Unit, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Unit<Vector3<f32>>,
}

impl Ray {
    pub fn transform(&self, transform: &Similarity3<f32>) -> Self {
        Self {
            origin: transform.transform_point(&self.origin),
            direction: Unit::new_normalize(transform.transform_vector(&self.direction.into_inner())),
        }
    }
}
