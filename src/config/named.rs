use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    config::{GeometryConfig, MaterialConfig, ShaderConfig},
    errors::SceneBuildError,
    geometry::GeometryEnum,
    material::MaterialEnum,
    shader::ShaderEnum,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Named<T> {
    Named(String),
    Inline(T),
}

impl<T> Named<T> {
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Named(name) => Some(name),
            Self::Inline(_) => None,
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
