use antler_scene::Style;
use antler_settings::{ImageSettings, LightingSettings, OcclusionSettings, ProbeSettings};

#[derive(Clone)]
pub struct RenderParameters {
    pub image: ImageSettings,
    pub occlusion: OcclusionSettings,
    pub lighting: LightingSettings,
    pub style: Style,
    pub probe: ProbeSettings,
}
