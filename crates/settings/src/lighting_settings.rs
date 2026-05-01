#[derive(Clone)]
pub struct LightingSettings {
    pub emitted: f32,
    pub ambient: f32,
    pub direct: f32,
    pub indirect: f32,
}

impl LightingSettings {
    #[must_use]
    #[inline]
    pub const fn new(emitted: f32, ambient: f32, direct: f32, indirect: f32) -> Self {
        assert!(emitted >= 0.0, "Emitted light must be non-negative");
        assert!(ambient >= 0.0, "Ambient light must be non-negative");
        assert!(direct >= 0.0, "Direct light must be non-negative");
        assert!(indirect >= 0.0, "Indirect light must be non-negative");

        Self {
            emitted,
            ambient,
            direct,
            indirect,
        }
    }
}
