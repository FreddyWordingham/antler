use std::collections::BTreeMap;

use antler_colour::Rgb;
use antler_parameters::SceneParameters;
use antler_scene::{Resources, Scene};
use serde::{Deserialize, Serialize};

use crate::{capture_config::CaptureConfig, light_config::LightConfig, object_config::ObjectConfig};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneConfig {
    pub ambient: Rgb,
    pub lights: Vec<LightConfig>,
    pub objects: Vec<ObjectConfig>,
    pub captures: BTreeMap<String, CaptureConfig>,
}

impl SceneConfig {
    pub fn build(self, resources: &mut Resources) -> SceneParameters {
        let mut scene = Scene::new();

        scene.set_ambient(self.ambient);

        for light in self.lights {
            scene.add_light(light.build());
        }

        for object in self.objects {
            scene.add_object(object.build(resources));
        }

        scene.build(resources);

        SceneParameters {
            scene,
            captures: self
                .captures
                .into_iter()
                .map(|(name, capture)| (name, capture.build()))
                .collect(),
        }
    }
}
