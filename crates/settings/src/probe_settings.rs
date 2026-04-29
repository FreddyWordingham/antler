#[derive(Clone)]
pub struct ProbeSettings {
    pub max_generation: u32,
    pub min_weight: f32,
}

impl ProbeSettings {
    #[must_use]
    #[inline]
    pub const fn new(max_generation: u32, min_weight: f32) -> Self {
        assert!(max_generation > 0, "Maximum generation must be positive");
        assert!(min_weight >= 0.0, "Minimum weight must be non-negative");

        Self {
            max_generation,
            min_weight,
        }
    }
}
