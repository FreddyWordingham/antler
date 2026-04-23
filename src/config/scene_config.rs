use std::{
    collections::BTreeMap,
    fs::{read_to_string, write},
};

use serde::{Deserialize, Serialize};

use crate::{
    config::{
        BuiltImage, GeometryConfig, ImageConfig, LightConfig, MaterialConfig, Named, ObjectConfig, RenderConfig,
        ShaderConfig,
    },
    errors::SceneBuildError,
    world::{Object, Scene, World},
};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct SceneConfig {
    #[serde(default)]
    pub image_defs: BTreeMap<String, ImageConfig>,
    #[serde(default)]
    pub light_defs: BTreeMap<String, LightConfig>,
    #[serde(default)]
    pub lights: Vec<Named<LightConfig>>,
    #[serde(default)]
    pub geometry_defs: BTreeMap<String, GeometryConfig>,
    #[serde(default)]
    pub shader_defs: BTreeMap<String, ShaderConfig>,
    #[serde(default)]
    pub material_defs: BTreeMap<String, MaterialConfig>,
    #[serde(default)]
    pub objects: Vec<ObjectConfig>,
}

pub struct BuiltScene {
    pub world: World,
    pub scene: Scene,
    pub images: BTreeMap<String, BuiltImage>,
}

impl SceneConfig {
    pub fn from_str(config_str: &str) -> Result<Self, SceneBuildError> {
        toml::from_str(config_str).map_err(|err| SceneBuildError::ConfigParseError(err.to_string()))
    }

    pub fn to_string(&self) -> Result<String, SceneBuildError> {
        toml::to_string(self).map_err(|err| SceneBuildError::ConfigParseError(err.to_string()))
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
            image_defs: images,
            light_defs,
            lights,
            geometry_defs,
            shader_defs,
            material_defs,
            objects,
        } = self;

        let mut world = World::new();
        let mut scene = Scene::new();

        for light in lights {
            scene.add_light(light.resolve(&light_defs)?);
        }

        for object in objects {
            let geometry = object.geometry.resolve(&geometry_defs)?;
            let shader = object.shader.resolve(&shader_defs)?;
            let material = object.material.resolve(&material_defs)?;

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
        for (image_name, image_entry) in images {
            if image_entry.renders.is_empty() {
                return Err(SceneBuildError::ImageHasNoRenders(image_name.clone()));
            }

            built_images.insert(
                image_name,
                BuiltImage {
                    background: image_entry.background,
                    camera: image_entry.camera.build(),
                    renders: image_entry.renders,
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
