use antler_geometry::Ray;

#[derive(Clone, Copy)]
pub struct Probe {
    pub ray: Ray,
    pub weight: f32,
    pub generation: u32,
}

impl Probe {
    #[must_use]
    pub const fn new(ray: Ray) -> Self {
        Self {
            ray,
            weight: 1.0,
            generation: 0,
        }
    }

    #[must_use]
    pub fn child(&self, ray: Ray, weight: f32) -> Self {
        Self {
            ray,
            weight: self.weight * weight,
            generation: self.generation + 1,
        }
    }
}
