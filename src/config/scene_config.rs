use std::{
    collections::BTreeMap,
    fs::{read_to_string, write},
};

use ron;
use serde::{Deserialize, Serialize};

use crate::{
    config::{BuiltImage, ImageConfig, LightConfig, Named, ObjectConfig},
    errors::SceneBuildError,
    world::{Object, Scene, World},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneConfig {
    #[serde(default)]
    pub images: Vec<Named<ImageConfig>>,
    #[serde(default)]
    pub lights: Vec<Named<LightConfig>>,
    #[serde(default)]
    pub objects: Vec<Named<ObjectConfig>>,
}

pub struct BuiltScene {
    pub world: World,
    pub scene: Scene,
    pub images: BTreeMap<String, BuiltImage>,
}

impl SceneConfig {
    pub fn from_str(config_str: &str) -> Result<Self, SceneBuildError> {
        ron::from_str(config_str).map_err(|err| SceneBuildError::ConfigParseError(err.to_string()))
    }

    pub fn to_string(&self) -> Result<String, SceneBuildError> {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .map_err(|err| SceneBuildError::ConfigParseError(err.to_string()))
    }

    pub fn load(path: &str) -> Result<Self, SceneBuildError> {
        let config_str = read_to_string(path).map_err(|err| SceneBuildError::ConfigParseError(err.to_string()))?;
        Self::from_str(&config_str)
    }

    pub fn save(&self, path: &str) -> Result<(), SceneBuildError> {
        let config_str = self.to_string()?;
        write(path, config_str).map_err(|err| SceneBuildError::ConfigParseError(err.to_string()))
    }

    pub fn build(self) -> Result<BuiltScene, SceneBuildError> {
        let SceneConfig {
            images,
            lights,
            objects,
        } = self;

        let mut world = World::new();
        let mut scene = Scene::new();

        for light in lights {
            let light = light.resolve("light")?;
            scene.add_light(light.build());
        }

        for object in objects {
            let object = object.resolve("object")?;

            let geometry = object.geometry.resolve("geometry")?.build()?;
            let shader = object.shader.resolve("shader")?.build();
            let material = object.material.resolve("material")?.build();

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

        let mut built_images = BTreeMap::new();

        for image in images {
            let image_name = match &image {
                Named::Named(name) => name.clone(),
                Named::Inline(_) => format!("image_{}", built_images.len()),
            };

            let image = image.resolve("image")?;

            if image.renders.is_empty() {
                return Err(SceneBuildError::ImageHasNoRenders(image_name));
            }

            built_images.insert(
                image_name,
                BuiltImage {
                    background: image.background,
                    camera: image.camera.build(),
                    renders: image.renders,
                },
            );
        }

        Ok(BuiltScene {
            world,
            scene,
            images: built_images,
        })
    }
}
