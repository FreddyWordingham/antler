use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    Config,
    context::Context,
    errors::ConfigError,
    named::Named,
    placeholder::{Placeholder, ResolvedPlaceholder},
    resolve::Resolve,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    #[serde(default = "default_assets_dir")]
    pub assets_dir: PathBuf,
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,
    pub placeholder: Named<Placeholder>,
}

#[derive(Debug)]
pub struct ResolvedManifest {
    pub assets_dir: PathBuf,
    pub output_dir: PathBuf,
    pub placeholder: ResolvedPlaceholder,
}

impl Manifest {
    pub fn load_resolved(path: impl AsRef<Path>) -> Result<ResolvedManifest, ConfigError> {
        let path = path.as_ref();
        let base_dir = path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();

        let manifest = Self::load(path)?;

        let context = Context {
            base_dir,
            assets_dir: manifest.assets_dir.clone(),
            output_dir: manifest.output_dir.clone(),
        };

        manifest.resolve(&context)
    }
}

impl Resolve for Manifest {
    type Resolved = ResolvedManifest;

    fn resolve(self, context: &Context) -> Result<Self::Resolved, ConfigError> {
        let assets_dir = self.assets_dir;
        let output_dir = self.output_dir;

        let context = Context {
            base_dir: context.base_dir.clone(),
            assets_dir: assets_dir.clone(),
            output_dir: output_dir.clone(),
        };

        Ok(Self::Resolved {
            assets_dir,
            output_dir,
            placeholder: self.placeholder.resolve(&context)?,
        })
    }
}

fn default_assets_dir() -> PathBuf {
    PathBuf::from("assets")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("output")
}
