use std::{
    collections::BTreeMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use antler_parameters::SimulationParameters;
use serde::{Deserialize, Serialize};

use crate::{errors::ConfigError, probe_config::ProbeConfig, scene_config::SceneConfig};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    #[serde(default = "default_assets_dir")]
    pub assets_dir: PathBuf,
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,
    pub scenes: BTreeMap<String, SceneConfig>,
    #[serde(default)]
    pub settings: ProbeConfig,
}

impl Manifest {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let contents = read_to_string(&path)?;
        let manifest = ron::from_str(&contents)?;
        Ok(manifest)
    }

    pub fn build(self) -> SimulationParameters {
        let mut resources = Default::default();

        let scenes = self
            .scenes
            .into_iter()
            .map(|(name, scene)| (name, scene.build(&mut resources)))
            .collect();

        SimulationParameters {
            assets_dir: self.assets_dir,
            output_dir: self.output_dir,
            resources,
            scenes,
            settings: self.settings.build(),
        }
    }
}

fn default_assets_dir() -> PathBuf {
    PathBuf::from("assets")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("output")
}
