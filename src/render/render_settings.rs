use crate::{colour::Rgb, config::RenderConfig};

#[derive(Debug, Clone, Copy)]
pub struct RenderSettings {
    pub resolution: [usize; 2],
    pub super_samples: usize,
    pub background: Rgb,
}

impl From<RenderConfig> for RenderSettings {
    fn from(value: RenderConfig) -> Self {
        Self {
            resolution: value.resolution,
            super_samples: value.super_samples,
            background: value.background,
        }
    }
}

impl From<&RenderConfig> for RenderSettings {
    fn from(value: &RenderConfig) -> Self {
        Self {
            resolution: value.resolution,
            super_samples: value.super_samples,
            background: value.background,
        }
    }
}
