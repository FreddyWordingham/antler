use nalgebra::Similarity3;
use serde::{Deserialize, Serialize};

use crate::{
    config::{GeometryConfig, MaterialConfig, ShaderConfig, Vec3, defaults},
    id::{GeometryId, MaterialId, ShaderId},
    world::{Object, World},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectConfig {
    pub geometry: GeometryConfig,
    pub shader: ShaderConfig,
    pub material: MaterialConfig,

    #[serde(default)]
    pub position: Vec3,

    #[serde(default)]
    pub rotation: Vec3,

    #[serde(default = "defaults::one")]
    pub scale: f32,
}

impl ObjectConfig {
    pub fn build(self, world: &mut World) -> Object {
        let geometry_id = world.add_geometry(self.geometry.into());
        let shader_id = world.add_shader(self.shader.into());
        let material_id = world.add_material(self.material.into());
        let transform = Similarity3::new(self.position.into(), self.rotation.into(), self.scale);

        Object::new(geometry_id, shader_id, material_id, transform)
    }
}
