use rand::{RngExt, SeedableRng, rngs::SmallRng};

use crate::tracing::WorldRay;

pub struct Probe {
    pub ray: WorldRay,
    pub weight: f32,
    pub generation: u32,
    rng: SmallRng,
}

impl Probe {
    pub fn new(ray: WorldRay) -> Self {
        Self::with_seed(ray, 0)
    }

    pub fn with_seed(ray: WorldRay, seed: u64) -> Self {
        Self {
            ray,
            weight: 1.0,
            generation: 0,
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    pub fn rng(&mut self) -> &mut SmallRng {
        &mut self.rng
    }

    pub fn child(&mut self, ray: WorldRay, weight: f32) -> Self {
        Self {
            ray,
            weight: self.weight * weight,
            generation: self.generation + 1,
            rng: SmallRng::seed_from_u64(self.rng.random::<u64>()),
        }
    }
}
