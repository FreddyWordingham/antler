use std::collections::BTreeMap;

use antler_parameters::CaptureParameters;
use serde::{Deserialize, Serialize};

use crate::{camera_config::CameraConfig, render_config::RenderConfig};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureConfig {
    pub camera: CameraConfig,
    pub renders: BTreeMap<String, RenderConfig>,
}

impl CaptureConfig {
    pub fn build(self) -> CaptureParameters {
        CaptureParameters {
            camera: self.camera.build(),
            renders: self
                .renders
                .into_iter()
                .map(|(name, render)| (name, render.build()))
                .collect(),
        }
    }
}
