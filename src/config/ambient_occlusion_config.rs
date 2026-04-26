use serde::{Deserialize, Serialize};

use crate::world::AmbientOcclusion;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AmbientOcclusionConfig {
    pub samples: usize,
    pub distance: f32,
    pub strength: f32,
}

impl AmbientOcclusionConfig {
    pub fn build(self) -> crate::world::AmbientOcclusion {
        AmbientOcclusion {
            samples: self.samples,
            distance: self.distance,
            strength: self.strength.clamp(0.0, 1.0),
        }
    }
}
