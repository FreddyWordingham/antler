use serde::{Deserialize, Serialize};

use crate::{
    camera::CameraEnum,
    config::{CameraConfig, LightConfig, ObjectConfig, RenderConfig},
    world::{Scene, World},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneConfig {
    pub render: RenderConfig,
    pub camera: CameraConfig,
    #[serde(default)]
    pub lights: Vec<LightConfig>,
    #[serde(default)]
    pub objects: Vec<ObjectConfig>,
}

pub struct BuiltScene {
    pub world: World,
    pub scene: Scene,
    pub camera: CameraEnum,
    pub render: crate::config::RenderConfig,
}

impl SceneConfig {
    pub fn build(self) -> BuiltScene {
        let mut world = World::new();
        let mut scene = Scene::new();

        for light in self.lights {
            scene.add_light(light);
        }

        for object in self.objects {
            scene.add_object(object.build(&mut world));
        }

        scene.build(&world);

        let aspect_ratio = self.render.resolution[0] as f32 / self.render.resolution[1] as f32;
        let camera = self.camera.build(aspect_ratio);

        BuiltScene {
            world,
            scene,
            camera,
            render: self.render,
        }
    }
}
