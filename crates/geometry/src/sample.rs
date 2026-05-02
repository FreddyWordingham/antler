use nalgebra::{Point3, Similarity3, Unit, Vector3};

pub struct Sample {
    pub position: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,
    pub pdf_area: f32,
}

impl Sample {
    #[must_use]
    #[inline]
    pub fn transform(&self, transform: &Similarity3<f32>) -> Self {
        let scale = transform.scaling();
        let area_scale = scale * scale;

        Self {
            position: transform.transform_point(&self.position),
            normal: Unit::new_normalize(transform.transform_vector(&self.normal)),
            pdf_area: self.pdf_area / area_scale,
        }
    }
}
