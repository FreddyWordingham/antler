use antler_id::{GeometryId, MaterialId, ShaderId};
use nalgebra::Similarity3;

pub struct Object {
    pub geometry_id: GeometryId,
    pub shader_id: ShaderId,
    pub material_id: MaterialId,
    pub transform: Similarity3<f32>,
    pub inv_transform: Similarity3<f32>,
}

impl Object {
    pub fn new(
        geometry_id: GeometryId,
        shader_id: ShaderId,
        material_id: MaterialId,
        transform: Similarity3<f32>,
    ) -> Self {
        let inv_transform = transform.inverse();
        Self {
            geometry_id,
            shader_id,
            material_id,
            transform,
            inv_transform,
        }
    }
}
