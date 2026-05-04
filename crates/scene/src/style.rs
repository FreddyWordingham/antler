use antler_colour::{Rgb, Rgba};

const BAYER_4X4: [[f32; 4]; 4] = [
    [0.0, 8.0, 2.0, 10.0],
    [12.0, 4.0, 14.0, 6.0],
    [3.0, 11.0, 1.0, 9.0],
    [15.0, 7.0, 13.0, 5.0],
];

#[derive(Clone)]
pub enum DitherMode {
    None,
    Bayer,
    Custom { values: Vec<f32>, size: [usize; 2] },
}

impl DitherMode {
    #[must_use]
    pub fn new_custom(values: Vec<f32>, size: [usize; 2]) -> Self {
        assert!(size[0] > 0 && size[1] > 0);
        assert_eq!(values.len(), size[0] * size[1]);

        Self::Custom { values, size }
    }

    #[must_use]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        !matches!(self, Self::None)
    }

    #[must_use]
    #[inline]
    pub fn sample(&self, pixel: [usize; 2]) -> f32 {
        match self {
            Self::None => 1.0,
            Self::Bayer => (BAYER_4X4[pixel[1] % 4][pixel[0] % 4] + 0.5) / 16.0,
            Self::Custom { values, size } => {
                let x = pixel[0] % size[0];
                let y = pixel[1] % size[1];

                values[y * size[0] + x].clamp(0.0, 1.0)
            }
        }
    }
}

#[derive(Clone)]
pub enum Style {
    Average,
    Banded { palette: Vec<Rgb>, dither: DitherMode },
}

impl Style {
    #[must_use]
    pub fn resolve(&self, pixel: [usize; 2], samples: &[Rgba]) -> Rgba {
        match self {
            Self::Average => resolve_average(samples),
            Self::Banded { palette, dither } => resolve_banded(pixel, samples, palette, dither),
        }
    }
}

#[must_use]
fn resolve_average(samples: &[Rgba]) -> Rgba {
    if samples.is_empty() {
        return Rgba::TRANSPARENT;
    }

    samples.iter().copied().sum::<Rgba>() / samples.len() as f32
}

#[must_use]
fn resolve_banded(pixel: [usize; 2], samples: &[Rgba], palette: &[Rgb], dither: &DitherMode) -> Rgba {
    if samples.is_empty() {
        return Rgba::TRANSPARENT;
    }

    if palette.is_empty() {
        return resolve_average(samples);
    }

    let average = resolve_average(samples);
    let alpha = average.alpha;

    if alpha <= 0.0 {
        return Rgba::TRANSPARENT;
    }

    let colour = average.to_rgb();

    let rgb = if dither.is_enabled() && palette.len() >= 2 {
        let [(a, da), (b, db)] = colour.nearest::<2>(palette);

        let da = da.sqrt();
        let db = db.sqrt();

        let t = da / (da + db + f32::EPSILON);

        if dither.sample(pixel) < t {
            palette[b]
        } else {
            palette[a]
        }
    } else {
        let [(index, _)] = colour.nearest::<1>(palette);
        palette[index]
    };

    let mut rgba = rgb.to_rgba();
    rgba.alpha = alpha;
    rgba
}
