use antler_colour::Rgb;
use antler_scene::Emissive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmissiveConfig {
    #[serde(default = "default_colour")]
    pub colour: Rgb,
    #[serde(default = "default_samples")]
    pub samples: usize,
}

impl EmissiveConfig {
    pub fn build(self) -> Emissive {
        Emissive::new(self.colour, self.samples)
    }
}

fn default_colour() -> Rgb {
    Rgb::WHITE
}

fn default_samples() -> usize {
    1
}
