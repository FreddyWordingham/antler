use crate::{colour::Rgba, config::RenderConfig};

#[derive(Debug, Clone, Copy)]
pub struct RenderSettings {
    pub background: Rgba,
    pub resolution: [usize; 2],
    pub super_samples: usize,
}

impl RenderSettings {
    pub fn from_config(config: &RenderConfig, default_background: Rgba) -> Self {
        Self {
            resolution: config.resolution,
            super_samples: config.super_samples,
            background: config.background.unwrap_or(default_background),
        }
    }
}
