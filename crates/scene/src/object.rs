use antler_id::{GeometryId, MaterialId, ShaderId};
use nalgebra::Similarity3;

use crate::emissive::Emissive;

pub struct Object {
    pub geometry_id: GeometryId,
    pub shader_id: ShaderId,
    pub material_id: MaterialId,
    pub emissive: Option<Emissive>,
    pub transform: Similarity3<f32>,
    pub inv_transform: Similarity3<f32>,
}

impl Object {
    #[must_use]
    pub fn new(
        geometry_id: GeometryId,
        shader_id: ShaderId,
        material_id: MaterialId,
        emissive: Option<Emissive>,
        transform: Similarity3<f32>,
    ) -> Self {
        let inv_transform = transform.inverse();
        Self {
            geometry_id,
            shader_id,
            material_id,
            emissive,
            transform,
            inv_transform,
        }
    }
}
