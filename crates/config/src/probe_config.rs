use antler_settings::ProbeSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeConfig {
    #[serde(default = "default_max_generation")]
    pub max_generation: u32,
    #[serde(default = "default_min_weight")]
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

fn default_max_generation() -> u32 {
    5
}

fn default_min_weight() -> f32 {
    0.01
}
