use serde::{Deserialize, Serialize};

use crate::{context::Context, errors::ConfigError, resolve::Resolve};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InnerPlaceholder {
    #[serde(default = "default_x")]
    pub x: i32,

    #[serde(default = "default_y")]
    pub y: i32,
}

impl Resolve for InnerPlaceholder {
    type Resolved = Self;

    fn resolve(self, _context: &Context) -> Result<Self::Resolved, ConfigError> {
        Ok(self)
    }
}

fn default_x() -> i32 {
    1
}

fn default_y() -> i32 {
    -2
}
