use nalgebra::Point3;

pub struct Aabb {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}
