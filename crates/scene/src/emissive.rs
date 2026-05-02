use antler_colour::Rgb;

pub struct Emissive {
    pub colour: Rgb,
    pub samples: usize,
}

impl Emissive {
    #[must_use]
    pub fn new(colour: Rgb, samples: usize) -> Self {
        Self {
            colour,
            samples: samples.max(1),
        }
    }
}
