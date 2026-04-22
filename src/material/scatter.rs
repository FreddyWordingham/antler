use crate::tracing::Photon;

pub struct Scatter {
    pub local_weight: f32,
    pub children: Vec<Photon>,
}

impl Scatter {
    pub fn total_weight(&self) -> f32 {
        self.local_weight + self.children.iter().map(|child| child.weight).sum::<f32>()
    }
}
