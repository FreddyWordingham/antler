use serde::{Deserialize, Serialize};

use crate::{colour::Rgba, config::defaults};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    pub resolution: [usize; 2],
    #[serde(default)]
    pub background: Option<Rgba>,
    #[serde(default = "defaults::one_usize")]
    pub super_samples: usize,
}
