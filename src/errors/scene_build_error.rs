use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use crate::errors::MeshLoadError;

#[derive(Debug)]
pub enum SceneBuildError {
    ConfigParseError(String),
    AssetLoadError { path: String, message: String },
    MeshLoad(MeshLoadError),
    ImageHasNoRenders(String),
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
            Self::AssetLoadError { path, message } => write!(f, "Failed to load asset at '{path}': {message}"),
            Self::MeshLoad(err) => write!(f, "{err}"),
            Self::ImageHasNoRenders(name) => write!(f, "Camera '{name}' has no renders defined"),
        }
    }
}

impl Error for SceneBuildError {}
