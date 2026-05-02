use antler_colour::Rgb;

pub struct Emissive {
    pub colour: Rgb,
    pub intensity: f32,
    pub samples: usize,
}

impl Emissive {
    #[must_use]
    pub fn new(colour: Rgb, intensity: f32, samples: usize) -> Self {
        Self {
            colour,
            intensity: intensity.max(0.0),
            samples: samples.max(1),
        }
    }
}
