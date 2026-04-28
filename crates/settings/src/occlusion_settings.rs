#[derive(Clone)]
pub struct OcclusionSettings {
    pub samples: usize,
    pub distance: f32,
    pub strength: f32,
}
