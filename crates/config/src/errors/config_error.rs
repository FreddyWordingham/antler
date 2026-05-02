use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
    path::PathBuf,
};

use antler_geometry::errors::MeshLoadError;
use antler_image::errors::ImageLoadError;
use ron::error::SpannedError;

use crate::errors::IncludeError;

#[derive(Debug)]
pub enum ConfigError {
    ParseError(String),
    Include(IncludeError),
    AssetLoadError { path: PathBuf, message: String },
    MeshLoad(MeshLoadError),
    ImageLoad(ImageLoadError),
}

impl From<IoError> for ConfigError {
    fn from(value: IoError) -> Self {
        Self::ParseError(format!("IO error: {value}"))
    }
}

impl From<SpannedError> for ConfigError {
    fn from(value: SpannedError) -> Self {
        Self::ParseError(format!("RON parse error: {value}"))
    }
}

impl From<IncludeError> for ConfigError {
    fn from(value: IncludeError) -> Self {
        Self::Include(value)
    }
}

impl From<MeshLoadError> for ConfigError {
    fn from(value: MeshLoadError) -> Self {
        Self::MeshLoad(value)
    }
}

impl From<ImageLoadError> for ConfigError {
    fn from(value: ImageLoadError) -> Self {
        Self::ImageLoad(value)
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ParseError(err) => write!(f, "Config parse error: {err}"),
            Self::Include(err) => write!(f, "Config include error: {err}"),
            Self::AssetLoadError { path, message } => {
                write!(f, "Failed to load asset at '{}': {}", path.display(), message)
            }
            Self::MeshLoad(err) => write!(f, "{err}"),
            Self::ImageLoad(err) => write!(f, "{err}"),
        }
    }
}

impl Error for ConfigError {}
