use std::collections::BTreeMap;

use crate::{camera::CameraEnum, colour::Rgba, config::render_config::RenderConfig};

pub struct BuiltImage {
    pub background: Rgba,
    pub camera: CameraEnum,
    pub renders: BTreeMap<String, RenderConfig>,
}
