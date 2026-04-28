use std::{
    fs::{read_to_string, write},
    path::Path,
};

use serde::{Serialize, de::DeserializeOwned};

use crate::errors::ConfigError;

pub trait Config: Sized + Serialize + DeserializeOwned {
    fn from_str(config_str: &str) -> Result<Self, ConfigError> {
        ron::from_str(config_str).map_err(|err| ConfigError::ParseError(err.to_string()))
    }

    fn to_str(&self) -> Result<String, ConfigError> {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .map_err(|err| ConfigError::ParseError(err.to_string()))
    }

    fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let config_str = read_to_string(path).map_err(|err| ConfigError::ParseError(err.to_string()))?;

        Self::from_str(&config_str)
    }

    fn save(&self, path: impl AsRef<Path>) -> Result<(), ConfigError> {
        let config_str = self.to_str()?;

        write(path, config_str).map_err(|err| ConfigError::ParseError(err.to_string()))
    }
}

impl<T> Config for T where T: Sized + Serialize + DeserializeOwned {}
