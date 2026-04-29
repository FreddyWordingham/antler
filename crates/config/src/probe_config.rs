use antler_settings::ProbeSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeConfig {
    pub max_generation: u32,
    pub min_weight: f32,
}

impl ProbeConfig {
    pub fn build(self) -> ProbeSettings {
        ProbeSettings {
            max_generation: self.max_generation,
            min_weight: self.min_weight,
        }
    }
}

impl Default for ProbeConfig {
    fn default() -> Self {
        Self {
            max_generation: 5,
            min_weight: 0.01,
        }
    }
}
