use crate::tracing::WorldRay;

pub struct Probe {
    pub ray: WorldRay,
    pub weight: f32,
    pub generation: u32,
}

impl Probe {
    pub fn new(ray: WorldRay) -> Self {
        Self {
            ray,
            weight: 1.0,
            generation: 0,
        }
    }

    pub fn child(&self, ray: WorldRay, weight: f32) -> Self {
        Self {
            ray,
            weight: self.weight * weight,
            generation: self.generation + 1,
        }
    }
}
