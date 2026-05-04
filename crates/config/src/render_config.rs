use antler_parameters::RenderParameters;
use serde::{Deserialize, Serialize};

use crate::{
    image_config::ImageConfig, lighting_config::LightingConfig, occlusion_config::OcclusionConfig,
    probe_config::ProbeConfig, style_config::StyleConfig,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenderConfig {
    #[serde(default)]
    occlusion: OcclusionConfig,
    #[serde(default)]
    lighting: LightingConfig,
    image: ImageConfig,
    #[serde(default)]
    style: StyleConfig,
    #[serde(default)]
    probe: ProbeConfig,
}

impl RenderConfig {
    pub fn build(self) -> RenderParameters {
        RenderParameters {
            occlusion: self.occlusion.build(),
            lighting: self.lighting.build(),
            image: self.image.build(),
            style: self.style.build(),
            probe: self.probe.build(),
        }
    }
}
