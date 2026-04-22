use crate::geometry::Ray;

pub struct Probe {
    pub ray: Ray,
    pub weight: f32,
    pub generation: u32,
}

impl Probe {
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            weight: 1.0,
            generation: 0,
        }
    }

    pub fn child(&self, ray: Ray, weight: f32) -> Self {
        Self {
            ray,
            weight: self.weight * weight,
            generation: self.generation + 1,
        }
    }
}
