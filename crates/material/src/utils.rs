use nalgebra::{Unit, Vector3};

#[inline]
pub fn reflect(incident: Unit<Vector3<f32>>, normal: Unit<Vector3<f32>>) -> Unit<Vector3<f32>> {
    Unit::new_normalize(*incident - 2.0 * incident.dot(&normal) * *normal)
}

#[inline]
pub fn refract(
    incident: Unit<Vector3<f32>>,
    normal: Unit<Vector3<f32>>,
    eta: f32,
    cos_theta: f32,
) -> Unit<Vector3<f32>> {
    let perpendicular = eta * (*incident + cos_theta * *normal);
    let parallel = -(1.0 - perpendicular.norm_squared()).abs().sqrt() * *normal;
    Unit::new_normalize(perpendicular + parallel)
}

#[inline]
pub fn schlick(cos_theta: f32, eta: f32) -> f32 {
    let r0 = ((1.0 - eta) / (1.0 + eta)).powi(2);
    (1.0 - r0).mul_add((1.0 - cos_theta).powi(5), r0)
}
