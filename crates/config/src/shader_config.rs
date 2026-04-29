use antler_colour::Rgb;
use antler_shader::{Block, Checkerboard, Luminous, Normal, Shader, Solid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ShaderConfig {
    Block { colour: Rgb },
    Checkerboard { size: f32, colour_a: Rgb, colour_b: Rgb },
    Luminous { colour: Rgb, intensity: f32 },
    Normal,
    Solid { colour: Rgb },
}

impl ShaderConfig {
    pub fn build(self) -> Shader {
        match self {
            Self::Block { colour } => Block::new(colour).into(),
            Self::Checkerboard {
                size,
                colour_a,
                colour_b,
            } => Checkerboard::new(size, colour_a, colour_b).into(),
            Self::Luminous { colour, intensity } => Luminous::new(colour, intensity).into(),
            Self::Normal => Normal.into(),
            Self::Solid { colour } => Solid::new(colour).into(),
        }
    }
}
