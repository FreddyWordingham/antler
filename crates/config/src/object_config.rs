use antler_scene::{Object, Resources};
use serde::{Deserialize, Serialize};

use crate::{
    errors::ConfigError, geometry_config::GeometryConfig, material_config::MaterialConfig, shader_config::ShaderConfig,
    transform::Transform,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectConfig {
    geometry: GeometryConfig,
    shader: ShaderConfig,
    material: MaterialConfig,
    #[serde(default)]
    transform: Transform,
}

impl ObjectConfig {
    pub fn build(self, resources: &mut Resources) -> Result<Object, ConfigError> {
        let geometry_id = resources.add_geometry(self.geometry.build()?);
        let shader_id = resources.add_shader(self.shader.build()?);
        let material_id = resources.add_material(self.material.build());

        Ok(Object::new(geometry_id, shader_id, material_id, self.transform.into()))
    }
}
