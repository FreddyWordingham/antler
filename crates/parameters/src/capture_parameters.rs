use std::collections::BTreeMap;

use antler_camera::Camera;
use antler_settings::ImageSettings;

pub struct CaptureParameters {
    pub camera: Camera,
    pub images: BTreeMap<String, ImageSettings>,
}
