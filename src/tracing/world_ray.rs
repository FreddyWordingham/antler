use std::ops::Deref;

use nalgebra::Similarity3;

use crate::{geometry::Ray, tracing::ObjectRay};

pub struct WorldRay(Ray);

impl WorldRay {
    pub fn new(ray: Ray) -> Self {
        Self(ray)
    }

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
