use antler_colour::Rgb;
use antler_scene::{DitherMode, Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum DitherModeConfig {
    #[default]
    None,
    Bayer,
    Custom {
        values: Vec<f32>,
        size: [usize; 2],
    },
}

impl DitherModeConfig {
    pub fn build(self) -> DitherMode {
        match self {
            Self::None => DitherMode::None,
            Self::Bayer => DitherMode::Bayer,
            Self::Custom { values, size } => DitherMode::new_custom(values, size),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum StyleConfig {
    #[default]
    Average,
    Banded {
        palette: Vec<Rgb>,
        #[serde(default)]
        dither: DitherModeConfig,
    },
}

impl StyleConfig {
    pub fn build(self) -> Style {
        match self {
            Self::Average => Style::Average,
            Self::Banded { palette, dither } => Style::Banded {
                palette,
                dither: dither.build(),
            },
        }
    }
}
