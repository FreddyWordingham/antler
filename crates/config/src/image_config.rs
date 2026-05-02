use antler_colour::Rgba;
use antler_settings::ImageSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageConfig {
    #[serde(default = "default_background")]
    pub background: Rgba,
    pub resolution: [usize; 2],
    #[serde(default = "default_tile_size")]
    pub tile_size: [usize; 2],
    #[serde(default = "default_super_samples")]
    pub super_samples: usize,
}

impl ImageConfig {
    pub const fn build(self) -> ImageSettings {
        ImageSettings {
            background: self.background,
            resolution: self.resolution,
            tile_size: self.tile_size,
            super_samples: self.super_samples,
        }
    }
}

const fn default_background() -> Rgba {
    Rgba::TRANSPARENT
}

const fn default_tile_size() -> [usize; 2] {
    [16, 16]
}

const fn default_super_samples() -> usize {
    1
}
