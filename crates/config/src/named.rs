use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{context::Context, errors::ConfigError, resolve::Resolve};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Named<T> {
    Named(PathBuf),
    Inline(T),
}

impl<T> Resolve for Named<T>
where
    T: Resolve + DeserializeOwned,
{
    type Resolved = T::Resolved;

    fn resolve(self, context: &Context) -> Result<Self::Resolved, ConfigError> {
        let config = match self {
            Self::Inline(value) => value,
            Self::Named(path) => load_named_config::<T>(&context.assets_dir, &path)?,
        };

        config.resolve(context)
    }
}

fn load_named_config<T>(assets_dir: &Path, path: &Path) -> Result<T, ConfigError>
where
    T: DeserializeOwned,
{
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        assets_dir.join(path)
    };

    let text = read_to_string(&path).map_err(|err| ConfigError::AssetLoadError {
        path: path.clone(),
        message: err.to_string(),
    })?;

    ron::from_str(&text)
        .map_err(|err| ConfigError::ParseError(format!("Failed to parse config from '{}': {err}", path.display())))
}
