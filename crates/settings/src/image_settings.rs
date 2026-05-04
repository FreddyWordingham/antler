use antler_colour::Rgba;

use crate::art_settings::ArtSettings;

#[derive(Clone)]
pub struct ImageSettings {
    pub background: Rgba,
    pub art_config: ArtSettings,
    pub resolution: [usize; 2],
    pub tile_size: [usize; 2],
    pub super_samples: usize,
}
