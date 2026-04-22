use crate::tracing::WorldRay;

pub struct Scatter {
    pub local_fraction: f32,
    pub children: Vec<(f32, WorldRay)>,
}

impl Scatter {
    pub fn total_fraction(&self) -> f32 {
        self.local_fraction + self.children.iter().map(|(fraction, _)| fraction).sum::<f32>()
    }

    pub fn is_energy_conserving(&self) -> bool {
        self.total_fraction() <= 1.0 + f32::EPSILON
    }
}
