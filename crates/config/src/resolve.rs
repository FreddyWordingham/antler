use crate::{context::Context, errors::ConfigError};

pub trait Resolve {
    type Resolved;

    fn resolve(self, context: &Context) -> Result<Self::Resolved, ConfigError>;
}
