use antler_colour::Rgba;

#[derive(Clone)]
pub struct RenderSettings {
    pub background: Rgba,
    pub resolution: [usize; 2],
    pub super_samples: usize,
}
