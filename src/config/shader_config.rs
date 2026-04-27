use serde::{Deserialize, Serialize};

use crate::{
    colour::{Gradient, Rgb},
    config::defaults,
    shader::{Block, Checkerboard, GradientShader, Lambertion, Luminous, ShaderEnum},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShaderConfig {
    Block {
        colour: Rgb,
    },
    GradientShader {
        stops: Vec<Rgb>,
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

impl ShaderConfig {
    pub fn build(self) -> ShaderEnum {
        match self {
            ShaderConfig::Block { colour } => Block::new(colour).into(),
            ShaderConfig::GradientShader { stops } => GradientShader::new(Gradient::new(stops)).into(),
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
