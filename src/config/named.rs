use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Named<T> {
    Named(String),
    Inline(T),
}

impl<T> Named<T> {
    pub fn resolve(self, registry: &BTreeMap<String, T>, kind: &'static str) -> Result<T, String>
    where
        T: Clone,
    {
        match self {
            Named::Inline(value) => Ok(value),
            Named::Named(name) => registry
                .get(&name)
                .cloned()
                .ok_or_else(|| format!("Unknown {} reference: '{}'.", kind, name)),
        }
    }
}
