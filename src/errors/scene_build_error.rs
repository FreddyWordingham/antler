use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use crate::errors::MeshLoadError;

#[derive(Debug)]
pub enum SceneBuildError {
    UnknownGeometry(String),
    UnknownShader(String),
    UnknownMaterial(String),
    MeshLoad(MeshLoadError),
}

impl From<MeshLoadError> for SceneBuildError {
    fn from(value: MeshLoadError) -> Self {
        Self::MeshLoad(value)
    }
}

impl Display for SceneBuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::UnknownGeometry(name) => write!(f, "Unknown geometry reference '{name}'"),
            Self::UnknownShader(name) => write!(f, "Unknown shader reference '{name}'"),
            Self::UnknownMaterial(name) => write!(f, "Unknown material reference '{name}'"),
            Self::MeshLoad(err) => write!(f, "{err}"),
        }
    }
}

impl Error for SceneBuildError {}
