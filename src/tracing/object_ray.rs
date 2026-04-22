use std::ops::Deref;

use nalgebra::Similarity3;

use crate::{geometry::Ray, tracing::WorldRay};

pub struct ObjectRay(Ray);

impl ObjectRay {
    pub fn new(ray: Ray) -> Self {
        Self(ray)
    }

    pub fn to_world_space(&self, transform: &Similarity3<f32>) -> WorldRay {
        WorldRay::new(self.0.transform(transform))
    }
}

impl Deref for ObjectRay {
    type Target = Ray;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
