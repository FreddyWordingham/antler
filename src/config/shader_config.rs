use serde::{Deserialize, Serialize};

use crate::{
    colour::Rgb,
    config::defaults,
    shader::{Block, Checkerboard, Lambertion, Luminous, ShaderEnum},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ShaderConfig {
    Block {
        colour: Rgb,
    },
    Lambertion {
        colour: Rgb,
    },
    Luminous {
        colour: Rgb,
        #[serde(default = "defaults::one_f32")]
        intensity: f32,
    },
    Checkerboard {
        #[serde(default = "defaults::one_f32")]
        size: f32,
        colour_a: Rgb,
        colour_b: Rgb,
    },
}

impl From<ShaderConfig> for ShaderEnum {
    fn from(config: ShaderConfig) -> Self {
        match config {
            ShaderConfig::Block { colour } => Block::new(colour).into(),
            ShaderConfig::Lambertion { colour } => Lambertion::new(colour).into(),
            ShaderConfig::Luminous { colour, intensity } => Luminous::new(colour, intensity).into(),
            ShaderConfig::Checkerboard {
                size,
                colour_a,
                colour_b,
            } => Checkerboard::new(size, colour_a, colour_b).into(),
        }
    }
}
