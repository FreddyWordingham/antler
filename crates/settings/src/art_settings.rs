use antler_colour::Rgb;

#[derive(Clone)]
pub enum ResolveMode {
    Average,
    PaletteAware { palette: Vec<Rgb>, dither: bool },
}

#[derive(Clone)]
pub struct ArtSettings {
    pub skybox_is_visible_to_camera: bool,
    pub resolve_mode: ResolveMode,
}
