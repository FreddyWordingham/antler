use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    path::PathBuf,
};

use antler_geometry::errors::MeshLoadError;

#[derive(Debug)]
pub enum ConfigError {
    ParseError(String),
    AssetLoadError { path: PathBuf, message: String },
    MeshLoad(MeshLoadError),
}

impl From<MeshLoadError> for ConfigError {
    fn from(value: MeshLoadError) -> Self {
        Self::MeshLoad(value)
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ParseError(err) => write!(f, "Config parse error: {err}"),
            Self::AssetLoadError { path, message } => {
                write!(f, "Failed to load asset at '{}': {}", path.display(), message)
            }
            Self::MeshLoad(err) => write!(f, "{err}"),
        }
    }
}

impl Error for ConfigError {}
