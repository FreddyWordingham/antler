use antler_colour::Rgba;

#[derive(Clone)]
pub struct ImageSettings {
    pub background: Rgba,
    pub skybox_is_visible: bool,
    pub resolution: [usize; 2],
    pub tile_size: [usize; 2],
    pub super_samples: usize,
}
