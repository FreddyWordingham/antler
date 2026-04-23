use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use crate::errors::MeshLoadError;

#[derive(Debug)]
pub enum SceneBuildError {
    ConfigParseError(String),
    UnknownCamera(String),
    UnknownLight(String),
    UnknownGeometry(String),
    UnknownShader(String),
    UnknownMaterial(String),
    MeshLoad(MeshLoadError),
    CameraHasNoRenders(String),
}

impl From<MeshLoadError> for SceneBuildError {
    fn from(value: MeshLoadError) -> Self {
        Self::MeshLoad(value)
    }
}

impl Display for SceneBuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ConfigParseError(err) => write!(f, "Config parse error: {err}"),
            Self::UnknownCamera(name) => write!(f, "Unknown camera reference '{name}'"),
            Self::UnknownLight(name) => write!(f, "Unknown light reference '{name}'"),
            Self::UnknownGeometry(name) => write!(f, "Unknown geometry reference '{name}'"),
            Self::UnknownShader(name) => write!(f, "Unknown shader reference '{name}'"),
            Self::UnknownMaterial(name) => write!(f, "Unknown material reference '{name}'"),
            Self::MeshLoad(err) => write!(f, "{err}"),
            Self::CameraHasNoRenders(name) => write!(f, "Camera '{name}' has no renders defined"),
        }
    }
}

impl Error for SceneBuildError {}
