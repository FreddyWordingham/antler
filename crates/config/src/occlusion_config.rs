use antler_settings::OcclusionSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OcclusionConfig {
    #[serde(default = "default_samples")]
    pub samples: usize,
    #[serde(default = "default_distance")]
    pub distance: f32,
    #[serde(default = "default_strength")]
    pub strength: f32,
    #[serde(default = "default_falloff")]
    pub falloff: f32,
}

impl OcclusionConfig {
    pub const fn build(self) -> OcclusionSettings {
        OcclusionSettings {
            samples: self.samples,
            distance: self.distance,
            strength: self.strength,
            falloff: self.falloff,
        }
    }
}

impl Default for OcclusionConfig {
    fn default() -> Self {
        Self {
            samples: default_samples(),
            distance: default_distance(),
            strength: default_strength(),
            falloff: default_falloff(),
        }
    }
}

const fn default_samples() -> usize {
    16
}

const fn default_distance() -> f32 {
    1.0
}

const fn default_strength() -> f32 {
    0.5
}

const fn default_falloff() -> f32 {
    1.0
}
