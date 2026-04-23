use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    camera::CameraEnum,
    config::{CameraConfig, GeometryConfig, LightConfig, MaterialConfig, ShaderConfig},
    errors::SceneBuildError,
    geometry::GeometryEnum,
    lighting::LightEnum,
    material::MaterialEnum,
    shader::ShaderEnum,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Named<T> {
    Named(String),
    Inline(T),
}

impl Named<CameraConfig> {
    pub fn resolve(self, registry: &BTreeMap<String, CameraConfig>) -> Result<CameraEnum, SceneBuildError> {
        match self {
            Named::Inline(config) => Ok(config.build()),
            Named::Named(name) => Ok(registry
                .get(&name)
                .cloned()
                .ok_or_else(|| SceneBuildError::UnknownCamera(name))?
                .build()),
        }
    }
}

impl Named<LightConfig> {
    pub fn resolve(self, registry: &BTreeMap<String, LightConfig>) -> Result<LightEnum, SceneBuildError> {
        match self {
            Named::Inline(config) => Ok(config.build()),

            Named::Named(name) => Ok(registry
                .get(&name)
                .cloned()
                .ok_or_else(|| SceneBuildError::UnknownLight(name))?
                .build()),
        }
    }
}

impl Named<GeometryConfig> {
    pub fn resolve(self, registry: &BTreeMap<String, GeometryConfig>) -> Result<GeometryEnum, SceneBuildError> {
        match self {
            Named::Inline(config) => Ok(config.build()?),
            Named::Named(name) => registry
                .get(&name)
                .cloned()
                .ok_or_else(|| SceneBuildError::UnknownGeometry(name))?
                .build()
                .map_err(SceneBuildError::from),
        }
    }
}

impl Named<ShaderConfig> {
    pub fn resolve(self, registry: &BTreeMap<String, ShaderConfig>) -> Result<ShaderEnum, SceneBuildError> {
        match self {
            Named::Inline(config) => Ok(config.build()),
            Named::Named(name) => Ok(registry
                .get(&name)
                .cloned()
                .ok_or_else(|| SceneBuildError::UnknownShader(name))?
                .build()),
        }
    }
}

impl Named<MaterialConfig> {
    pub fn resolve(self, registry: &BTreeMap<String, MaterialConfig>) -> Result<MaterialEnum, SceneBuildError> {
        match self {
            Named::Inline(config) => Ok(config.build()),
            Named::Named(name) => Ok(registry
                .get(&name)
                .cloned()
                .ok_or_else(|| SceneBuildError::UnknownMaterial(name))?
                .build()),
        }
    }
}
