use antler_colour::{Rgb, Rgba};

const BAYER_2X2: [f32; 4] = [0.125, 0.625, 0.875, 0.375];
const BAYER_4X4: [f32; 16] = [
    0.03125, 0.53125, 0.15625, 0.65625, 0.78125, 0.28125, 0.90625, 0.40625, 0.21875, 0.71875, 0.09375, 0.59375,
    0.96875, 0.46875, 0.84375, 0.34375,
];
const CHECKER_2X2: [f32; 4] = [0.25, 0.75, 0.75, 0.25];
const STRIPE_4X4: [f32; 16] = [
    0.125, 0.375, 0.625, 0.875, 0.125, 0.375, 0.625, 0.875, 0.125, 0.375, 0.625, 0.875, 0.125, 0.375, 0.625, 0.875,
];
const DIAGONAL_4X4: [f32; 16] = [
    0.125, 0.375, 0.625, 0.875, 0.875, 0.125, 0.375, 0.625, 0.625, 0.875, 0.125, 0.375, 0.375, 0.625, 0.875, 0.125,
];

#[derive(Clone)]
pub enum DitherMode {
    None,
    Bayer2x2,
    Bayer4x4,
    Checker2x2,
    Stripe4x4,
    Diagonal4x4,
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
    pub const fn is_enabled(&self) -> bool {
        !matches!(self, Self::None)
    }

    #[must_use]
    #[inline]
    pub fn sample(&self, pixel: [usize; 2]) -> f32 {
        match self {
            Self::None => 1.0,
            Self::Bayer2x2 => Self::sample_matrix(pixel, &BAYER_2X2, [2, 2]),
            Self::Bayer4x4 => Self::sample_matrix(pixel, &BAYER_4X4, [4, 4]),
            Self::Checker2x2 => Self::sample_matrix(pixel, &CHECKER_2X2, [2, 2]),
            Self::Stripe4x4 => Self::sample_matrix(pixel, &STRIPE_4X4, [4, 4]),
            Self::Diagonal4x4 => Self::sample_matrix(pixel, &DIAGONAL_4X4, [4, 4]),
            Self::Custom { values, size } => Self::sample_matrix(pixel, values, *size),
        }
    }

    #[must_use]
    #[inline]
    fn sample_matrix(pixel: [usize; 2], values: &[f32], size: [usize; 2]) -> f32 {
        let x = pixel[0] % size[0];
        let y = pixel[1] % size[1];
        values[y * size[0] + x].clamp(0.0, 1.0)
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
