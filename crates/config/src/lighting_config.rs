use antler_settings::LightingSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LightingConfig {
    #[serde(default = "default_emitted")]
    pub emitted: f32,
    #[serde(default = "default_ambient")]
    pub ambient: f32,
    #[serde(default = "default_direct")]
    pub direct: f32,
    #[serde(default = "default_indirect")]
    pub indirect: f32,
}

impl LightingConfig {
    pub fn build(self) -> LightingSettings {
        LightingSettings {
            emitted: self.emitted,
            ambient: self.ambient,
            direct: self.direct,
            indirect: self.indirect,
        }
    }
}

impl Default for LightingConfig {
    fn default() -> Self {
        Self {
            emitted: default_emitted(),
            ambient: default_ambient(),
            direct: default_direct(),
            indirect: default_indirect(),
        }
    }
}

fn default_emitted() -> f32 {
    1.0
}

fn default_ambient() -> f32 {
    0.1
}

fn default_direct() -> f32 {
    1.0
}

fn default_indirect() -> f32 {
    1.0
}
