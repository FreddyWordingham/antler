use std::path::PathBuf;

use antler_colour::Rgb;
use antler_image::RgbImage;
use antler_shader::{
    Angular, Block, Checkerboard, Gradient, Iridescent, Luminous, Normal, Shader, Solid, Textured, Wireframe,
};
use serde::{Deserialize, Serialize};

use crate::{errors::ConfigError, gradient_config::GradientConfig, vec3::Vec3};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ShaderConfig {
    Angular {
        gradient: GradientConfig,
        power: f32,
        #[serde(default = "default_direction")]
        direction: Vec3,
    },
    Block {
        colour: Rgb,
    },
    Checkerboard {
        size: f32,
        colour_a: Rgb,
        colour_b: Rgb,
    },
    Gradient {
        gradient: GradientConfig,
        power: f32,
    },
    Iridescent {
        gradient: GradientConfig,
        power: f32,
    },
    Luminous {
        colour: Rgb,
        intensity: f32,
    },
    Normal,
    Solid {
        colour: Rgb,
    },
    Textured {
        path: PathBuf,
    },
    Wireframe {
        surface_colour: Rgb,
        line_colour: Rgb,
        width: f32,
    },
}

impl ShaderConfig {
    pub fn build(self) -> Result<Shader, ConfigError> {
        Ok(match self {
            Self::Angular {
                gradient,
                power,
                direction,
            } => Angular::new(gradient.into(), power, direction.into()).into(),
            Self::Block { colour } => Block::new(colour).into(),
            Self::Checkerboard {
                size,
                colour_a,
                colour_b,
            } => Checkerboard::new(size, colour_a, colour_b).into(),
            Self::Gradient { gradient, power } => Gradient::new(gradient.into(), power).into(),
            Self::Iridescent { gradient, power } => Iridescent::new(gradient.into(), power).into(),
            Self::Luminous { colour, intensity } => Luminous::new(colour, intensity).into(),
            Self::Normal => Normal::new().into(),
            Self::Solid { colour } => Solid::new(colour).into(),
            Self::Textured { path } => Textured::new(RgbImage::load(path)?).into(),
            Self::Wireframe {
                surface_colour,
                line_colour,
                width,
            } => Wireframe::new(surface_colour, line_colour, width).into(),
        })
    }
}

fn default_direction() -> Vec3 {
    Vec3::new(0.0, 0.0, 1.0)
}
