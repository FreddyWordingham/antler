use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    colour::Rgba,
    config::{CameraConfig, RenderConfig, defaults},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    #[serde(default = "defaults::transparent")]
    pub background: Rgba,
    #[serde(flatten)]
    pub camera: CameraConfig,
    #[serde(default)]
    pub renders: BTreeMap<String, RenderConfig>,
}
