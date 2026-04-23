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
        #[serde(default = "defaults::one")]
        intensity: f32,
    },
    Checkerboard {
        #[serde(default = "defaults::one")]
        size: f32,
        colour_a: Rgb,
        colour_b: Rgb,
    },
}

impl From<ShaderConfig> for ShaderEnum {
    fn from(config: ShaderConfig) -> Self {
        match config {
            ShaderConfig::Block { colour } => ShaderEnum::Block(Block { colour }),
            ShaderConfig::Lambertion { colour } => ShaderEnum::Lambertion(Lambertion { colour }),
            ShaderConfig::Luminous { colour, intensity } => ShaderEnum::Luminous(Luminous { colour, intensity }),
            ShaderConfig::Checkerboard {
                size,
                colour_a,
                colour_b,
            } => ShaderEnum::Checkerboard(Checkerboard {
                size,
                colour_a,
                colour_b,
            }),
        }
    }
}
