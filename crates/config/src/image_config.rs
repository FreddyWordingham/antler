use antler_colour::Rgba;
use antler_settings::ImageSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageConfig {
    pub background: Rgba,
    pub resolution: [usize; 2],
    pub tile_size: [usize; 2],
    pub super_samples: usize,
}

impl ImageConfig {
    pub fn build(self) -> ImageSettings {
        ImageSettings {
            background: self.background,
            resolution: self.resolution,
            tile_size: self.tile_size,
            super_samples: self.super_samples,
        }
    }
}
