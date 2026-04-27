use nalgebra::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    min: Point3<f32>,
    max: Point3<f32>,
}

impl Aabb {
    #[must_use]
    pub fn new(min: Point3<f32>, max: Point3<f32>) -> Self {
        assert!(min.x <= max.x);
        assert!(min.y <= max.y);
        assert!(min.z <= max.z);
        Self { min, max }
    }
}
