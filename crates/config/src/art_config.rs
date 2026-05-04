use antler_colour::Rgb;
use antler_settings::{ArtSettings, ResolveMode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum ResolveModeConfig {
    #[default]
    Average,
    PaletteAware {
        palette: Vec<Rgb>,
        dither: bool,
    },
}

impl ResolveModeConfig {
    pub fn build(self) -> ResolveMode {
        match self {
            Self::Average => ResolveMode::Average,
            Self::PaletteAware { palette, dither } => ResolveMode::PaletteAware { palette, dither },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtConfig {
    #[serde(default)]
    pub skybox_is_visible_to_camera: bool,
    #[serde(default)]
    pub resolve_mode: ResolveModeConfig,
}

impl Default for ArtConfig {
    fn default() -> Self {
        Self {
            skybox_is_visible_to_camera: true,
            resolve_mode: ResolveModeConfig::Average,
        }
    }
}

impl ArtConfig {
    pub fn build(self) -> ArtSettings {
        ArtSettings {
            skybox_is_visible_to_camera: self.skybox_is_visible_to_camera,
            resolve_mode: self.resolve_mode.build(),
        }
    }
}
