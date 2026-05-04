use std::collections::BTreeMap;

use antler_camera::Camera;

use crate::render_parameters::RenderParameters;

pub struct CaptureParameters {
    pub camera: Camera,
    pub renders: BTreeMap<String, RenderParameters>,
}
