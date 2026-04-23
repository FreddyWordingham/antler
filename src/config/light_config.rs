use serde::{Deserialize, Serialize};

use crate::{colour::Rgb, config::Vec3};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LightConfig {
    Directional { direction: Vec3, radiance: Rgb },
}
