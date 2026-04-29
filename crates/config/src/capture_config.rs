use std::collections::BTreeMap;

use antler_parameters::CaptureParameters;
use serde::{Deserialize, Serialize};

use crate::{camera_config::CameraConfig, image_config::ImageConfig};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureConfig {
    pub camera: CameraConfig,
    pub images: BTreeMap<String, ImageConfig>,
}

impl CaptureConfig {
    pub fn build(self) -> CaptureParameters {
        CaptureParameters {
            camera: self.camera.build(),
            images: self
                .images
                .into_iter()
                .map(|(name, image)| (name, image.build()))
                .collect(),
        }
    }
}
