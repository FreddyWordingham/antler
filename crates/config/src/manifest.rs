use std::{
    collections::BTreeMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use antler_parameters::SimulationParameters;
use serde::{Deserialize, Serialize};

use crate::{
    errors::ConfigError, lighting_config::LightingConfig, probe_config::ProbeConfig, scene_config::SceneConfig,
    utils::expand_includes,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    #[serde(default = "default_assets_dir")]
    pub assets_dir: PathBuf,
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,
    pub scenes: BTreeMap<String, SceneConfig>,
    #[serde(default)]
    pub lighting_settings: LightingConfig,
    #[serde(default)]
    pub probe_settings: ProbeConfig,
}

impl Manifest {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let contents = read_to_string(&path)?;

        let cwd = std::env::current_dir()?;
        let manifest_ron = expand_includes(&contents, cwd)?;

        let manifest = ron::from_str(&manifest_ron)?;
        Ok(manifest)
    }

    pub fn build(self) -> Result<SimulationParameters, ConfigError> {
        let mut resources = Default::default();

        let scenes = self
            .scenes
            .into_iter()
            .map(|(name, scene)| {
                let scene = scene.build(&mut resources)?;
                Ok((name, scene))
            })
            .collect::<Result<_, ConfigError>>()?;

        Ok(SimulationParameters {
            assets_dir: self.assets_dir,
            output_dir: self.output_dir,
            resources,
            scenes,
            lighting_settings: self.lighting_settings.build(),
            probe_settings: self.probe_settings.build(),
        })
    }
}

fn default_assets_dir() -> PathBuf {
    PathBuf::from("assets")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("output")
}
