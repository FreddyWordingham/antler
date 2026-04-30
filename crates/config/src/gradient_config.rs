use antler_colour::{Rgb, RgbGradient};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct GradientConfig(pub RgbGradient);

impl From<GradientConfig> for RgbGradient {
    fn from(value: GradientConfig) -> Self {
        value.0
    }
}

impl Serialize for GradientConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.stops().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for GradientConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let stops = Vec::<Rgb>::deserialize(deserializer)?;
        Ok(Self(RgbGradient::new(stops)))
    }
}
