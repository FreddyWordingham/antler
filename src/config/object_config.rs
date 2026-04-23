use serde::{Deserialize, Serialize};

use crate::config::{GeometryConfig, MaterialConfig, Named, ShaderConfig, TransformConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectConfig {
    pub geometry: Named<GeometryConfig>,
    pub shader: Named<ShaderConfig>,
    pub material: Named<MaterialConfig>,
    #[serde(default)]
    pub transform: TransformConfig,
}
