use serde::{Deserialize, Serialize};

use crate::{
    context::Context, errors::ConfigError, inner_placeholder::InnerPlaceholder, named::Named, resolve::Resolve,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Placeholder {
    #[serde(default = "default_x")]
    pub x: i32,

    #[serde(default = "default_y")]
    pub y: i32,

    pub inner: Named<InnerPlaceholder>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolvedPlaceholder {
    pub x: i32,
    pub y: i32,
    pub inner: InnerPlaceholder,
}

impl Resolve for Placeholder {
    type Resolved = ResolvedPlaceholder;

    fn resolve(self, context: &Context) -> Result<Self::Resolved, ConfigError> {
        let x = self.x;
        let y = self.y;

        Ok(Self::Resolved {
            x,
            y,
            inner: self.inner.resolve(context)?,
        })
    }
}

fn default_x() -> i32 {
    1
}

fn default_y() -> i32 {
    -2
}
