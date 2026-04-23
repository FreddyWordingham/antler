use std::{
    collections::BTreeMap,
    fs::{read_to_string, write},
};

use serde::{Deserialize, Serialize};

use crate::{
    camera::CameraEnum,
    config::{CameraConfig, GeometryConfig, LightConfig, MaterialConfig, ObjectConfig, RenderConfig, ShaderConfig},
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
    pub geometries: BTreeMap<String, GeometryConfig>,
    #[serde(default)]
    pub shaders: BTreeMap<String, ShaderConfig>,
    #[serde(default)]
    pub materials: BTreeMap<String, MaterialConfig>,
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
            render,
            camera,
            lights,
            geometries,
            shaders,
            materials,
            objects,
        } = self;

        let mut world = World::new();
        let mut scene = Scene::new();

        for light in lights {
            scene.add_light(light.build());
        }

        for object in objects {
            let geometry = object.geometry.resolve(&geometries)?;
            let shader = object.shader.resolve(&shaders)?;
            let material = object.material.resolve(&materials)?;

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

        let aspect_ratio = render.resolution[0] as f32 / render.resolution[1] as f32;
        let camera = camera.build(aspect_ratio);

        Ok(BuiltScene {
            world,
            scene,
            camera,
            render,
        })
    }
}
