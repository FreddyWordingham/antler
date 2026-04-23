use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    camera::CameraEnum,
    config::{CameraConfig, LightConfig, Named, ObjectConfig, RenderConfig},
    errors::SceneBuildError,
    world::{Object, Scene, World},
};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct SceneConfig {
    pub render: RenderConfig,
    pub camera: CameraConfig,
    #[serde(default)]
    pub lights: Vec<LightConfig>,
    #[serde(default)]
    pub geometries: BTreeMap<String, crate::config::GeometryConfig>,
    #[serde(default)]
    pub shaders: BTreeMap<String, crate::config::ShaderConfig>,
    #[serde(default)]
    pub materials: BTreeMap<String, crate::config::MaterialConfig>,
    #[serde(default)]
    pub objects: Vec<ObjectConfig>,
}

pub struct BuiltScene {
    pub world: World,
    pub scene: Scene,
    pub camera: CameraEnum,
    pub render: RenderConfig,
}

impl SceneConfig {
    pub fn build(self) -> Result<BuiltScene, SceneBuildError> {
        let mut world = World::new();
        let mut scene = Scene::new();

        for light in self.lights {
            scene.add_light(light.build());
        }

        for object in self.objects {
            let geometry = match object.geometry {
                Named::Inline(config) => config.build()?,
                Named::Named(name) => self
                    .geometries
                    .get(&name)
                    .cloned()
                    .ok_or(SceneBuildError::UnknownGeometry(name))?
                    .build()?,
            };

            let shader = match object.shader {
                Named::Inline(config) => config.build(),
                Named::Named(name) => self
                    .shaders
                    .get(&name)
                    .cloned()
                    .ok_or(SceneBuildError::UnknownShader(name))?
                    .build(),
            };

            let material = match object.material {
                Named::Inline(config) => config.build(),
                Named::Named(name) => self
                    .materials
                    .get(&name)
                    .cloned()
                    .ok_or(SceneBuildError::UnknownMaterial(name))?
                    .build(),
            };

            let geometry_id = world.add_geometry(geometry);
            let shader_id = world.add_shader(shader);
            let material_id = world.add_material(material);

            scene.add_object(Object::new(
                geometry_id,
                shader_id,
                material_id,
                object.transform.into(),
            ));
        }

        scene.build(&world);

        let aspect_ratio = self.render.resolution[0] as f32 / self.render.resolution[1] as f32;
        let camera = self.camera.build(aspect_ratio);

        Ok(BuiltScene {
            world,
            scene,
            camera,
            render: self.render,
        })
    }
}
