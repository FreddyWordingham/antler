use std::fs::read_to_string;

use ron;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::errors::SceneBuildError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Named<T> {
    Named(String),
    Inline(T),
}

impl<T> Named<T>
where
    T: DeserializeOwned,
{
    pub fn resolve(self, asset_type: &str) -> Result<T, SceneBuildError> {
        match self {
            Self::Inline(value) => Ok(value),
            Self::Named(name) => {
                let path = format!("assets/{asset_type}/{name}.ron");

                let text = read_to_string(&path).map_err(|err| SceneBuildError::AssetLoadError {
                    path: path.clone(),
                    message: err.to_string(),
                })?;

                ron::from_str(&text).map_err(|err| SceneBuildError::AssetLoadError {
                    path,
                    message: err.to_string(),
                })
            }
        }
    }
}
