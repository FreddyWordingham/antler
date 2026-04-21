use nalgebra::Affine3;

use crate::id::{GeometryId, MaterialId, ShaderId};

pub struct Object {
    pub transform: Affine3<f32>,
    pub geometry_id: GeometryId,
    pub shader_id: ShaderId,
    pub material_id: MaterialId,
}
