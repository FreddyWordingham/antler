use serde::{Deserialize, Serialize};

use crate::{
    config::{GeometryConfig, MaterialConfig, ShaderConfig, TransformConfig},
    world::{Object, World},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectConfig {
    pub geometry: GeometryConfig,
    pub shader: ShaderConfig,
    pub material: MaterialConfig,

    #[serde(default)]
    pub transform: TransformConfig,
}

impl ObjectConfig {
    pub fn build(self, world: &mut World) -> Object {
        let geometry_id = world.add_geometry(self.geometry);
        let shader_id = world.add_shader(self.shader);
        let material_id = world.add_material(self.material);
        let transform = self.transform.into();

        Object::new(geometry_id, shader_id, material_id, transform)
    }
}
