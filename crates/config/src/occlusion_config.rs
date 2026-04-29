use antler_settings::OcclusionSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OcclusionConfig {
    pub samples: usize,
    pub distance: f32,
    pub strength: f32,
}

impl OcclusionConfig {
    pub fn build(self) -> OcclusionSettings {
        OcclusionSettings {
            samples: self.samples,
            distance: self.distance,
            strength: self.strength,
        }
    }
}

impl Default for OcclusionConfig {
    fn default() -> Self {
        Self {
            samples: 16,
            distance: 1.0,
            strength: 0.5,
        }
    }
}
