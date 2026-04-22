use nalgebra::{Point3, Similarity3};

use crate::{
    geometry::{Bounded, Ray, Traceable},
    tracing::Hit,
};

#[derive(Debug, Clone)]
pub struct Aabb {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl Aabb {
    pub fn new(min: Point3<f32>, max: Point3<f32>) -> Self {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        assert!(min.z <= max.z);
        Self { min, max }
    }

    pub fn vertices(&self) -> [Point3<f32>; 8] {
        [
            self.min,
            Point3::new(self.max.x, self.min.y, self.min.z),
            Point3::new(self.min.x, self.max.y, self.min.z),
            Point3::new(self.max.x, self.max.y, self.min.z),
            Point3::new(self.min.x, self.min.y, self.max.z),
            Point3::new(self.max.x, self.min.y, self.max.z),
            Point3::new(self.min.x, self.max.y, self.max.z),
            self.max,
        ]
    }

    pub fn transform(&self, transform: &Similarity3<f32>) -> Self {
        let transformed_vertices = self.vertices().into_iter().map(|v| transform.transform_point(&v));
        let min = transformed_vertices
            .clone()
            .fold(Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY), |a, b| {
                Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
            });
        let max = transformed_vertices.fold(
            Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
            |a, b| Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)),
        );
        Self { min, max }
    }
}

impl Bounded for Aabb {
    fn bounds(&self) -> Aabb {
        self.clone()
    }
}

impl Traceable for Aabb {
    fn trace(&self, ray: &Ray) -> Option<Hit> {
        todo!()
    }
}
