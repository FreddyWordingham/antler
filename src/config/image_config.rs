use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::config::{CameraConfig, RenderConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    #[serde(flatten)]
    pub camera: CameraConfig,

    #[serde(default)]
    pub renders: BTreeMap<String, RenderConfig>,
}
