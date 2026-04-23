use std::{
    error::Error,
    fs::{read_to_string, write},
};

use serde::{Deserialize, Serialize};
use toml::{de::Error as TomlDeError, ser::Error as TomlSerError};

use crate::{colour::Rgb, config::GeometryConfig};

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub width: usize,
    pub height: usize,
    pub background: Rgb,
    pub geometry: GeometryConfig,
}

impl Manifest {
    #[inline]
    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let contents = self.to_string()?;
        write(path, contents)?;
        Ok(())
    }

    #[inline]
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = read_to_string(path)?;
        Ok(Self::from_str(&contents)?)
    }

    #[inline]
    pub fn to_string(&self) -> Result<String, TomlSerError> {
        toml::to_string(self)
    }
    #[inline]
    pub fn from_str(s: &str) -> Result<Self, TomlDeError> {
        toml::from_str(s)
    }
}
