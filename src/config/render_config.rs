use serde::{Deserialize, Serialize};

use crate::{colour::Rgb, config::defaults};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    pub resolution: [usize; 2],
    #[serde(default = "defaults::black")]
    pub background: Rgb,
    #[serde(default = "defaults::one_usize")]
    pub super_samples: usize,
}
