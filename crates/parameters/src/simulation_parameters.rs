use std::{collections::BTreeMap, path::PathBuf};

use antler_scene::Resources;
use antler_settings::{LightingSettings, ProbeSettings};

use crate::scene_parameters::SceneParameters;

pub struct SimulationParameters {
    pub assets_dir: PathBuf,
    pub output_dir: PathBuf,
    pub resources: Resources,
    pub scenes: BTreeMap<String, SceneParameters>,
    pub lighting_settings: LightingSettings,
    pub probe_settings: ProbeSettings,
}
