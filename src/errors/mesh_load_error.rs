use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

use tobj::LoadError;

#[derive(Debug)]
pub enum MeshLoadError {
    Obj(LoadError),
    EmptyMesh,
    MissingPositionIndex { model_name: String, index: usize },
    MissingNormalIndex { model_name: String, index: usize },
    MissingTexcoordIndex { model_name: String, index: usize },
    InvalidIndexCount { model_name: String, count: usize },
}

impl From<LoadError> for MeshLoadError {
    fn from(value: LoadError) -> Self {
        Self::Obj(value)
    }
}

impl Display for MeshLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Obj(err) => write!(f, "Failed to load OBJ: {err}"),
            Self::EmptyMesh => write!(f, "OBJ produced no triangles"),
            Self::MissingPositionIndex { model_name, index } => {
                write!(f, "Model '{model_name}' referenced missing vertex index {index}")
            }
            Self::MissingNormalIndex { model_name, index } => {
                write!(f, "Model '{model_name}' referenced missing normal index {index}")
            }
            Self::MissingTexcoordIndex { model_name, index } => {
                write!(f, "Model '{model_name}' referenced missing texcoord index {index}")
            }
            Self::InvalidIndexCount { model_name, count } => {
                write!(f, "Model '{model_name}' has non-triangle index count {count}")
            }
        }
    }
}

impl Error for MeshLoadError {}
