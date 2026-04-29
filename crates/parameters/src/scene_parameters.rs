use std::collections::BTreeMap;

use antler_scene::Scene;

use crate::capture_parameters::CaptureParameters;

pub struct SceneParameters {
    pub scene: Scene,
    pub captures: BTreeMap<String, CaptureParameters>,
}
