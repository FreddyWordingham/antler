use std::ops::{Add, Mul};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gradient<C> {
    stops: Vec<C>,
}

impl<C> Gradient<C> {
    pub fn new(stops: Vec<C>) -> Self {
        assert!(!stops.is_empty(), "Gradient must have at least one stop.");
        Self { stops }
    }

    pub fn stops(&self) -> &[C] {
        &self.stops
    }
}

impl<C> Gradient<C>
where
    C: Copy + Add<Output = C> + Mul<f32, Output = C>,
{
    #[inline]
    pub fn sample(&self, t: f32) -> C {
        assert!(!self.stops.is_empty(), "Cannot sample an empty gradient.");

        if self.stops.len() == 1 {
            return self.stops[0];
        }

        let t = t.clamp(0.0, 1.0);
        let scaled = t * (self.stops.len() - 1) as f32;

        let i0 = scaled.floor() as usize;
        let i1 = (i0 + 1).min(self.stops.len() - 1);
        let alpha = scaled - i0 as f32;

        self.stops[i0] * (1.0 - alpha) + self.stops[i1] * alpha
    }
}
