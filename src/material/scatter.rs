use crate::geometry::Ray;

pub struct Scatter {
    pub local_weight: f32,
    pub children: Vec<(f32, Ray)>,
}
