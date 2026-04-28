use std::f32::consts::PI;

use nalgebra::{Point3, Unit, Vector3};
use rand::{Rng, RngExt};

pub const RAY_BIAS: f32 = 1e-4;

pub fn offset_origin(position: Point3<f32>, normal: Unit<Vector3<f32>>, direction: Unit<Vector3<f32>>) -> Point3<f32> {
    if direction.dot(&normal) >= 0.0 {
        position + *normal * RAY_BIAS
    } else {
        position - *normal * RAY_BIAS
    }
}

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
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

#[inline]
pub fn cosine_weighted_hemisphere<R: Rng>(rng: &mut R, normal: Unit<Vector3<f32>>) -> Unit<Vector3<f32>> {
    let u1: f32 = rng.random();
    let u2: f32 = rng.random();

    let r = u1.sqrt();
    let theta = 2.0 * PI * u2;

    let x = r * theta.cos();
    let y = r * theta.sin();
    let z = (1.0 - u1).sqrt();

    let (tangent, bitangent) = tangent_frame(normal);

    Unit::new_normalize(tangent * x + bitangent * y + *normal * z)
}

#[inline]
pub fn tangent_frame(normal: Unit<Vector3<f32>>) -> (Vector3<f32>, Vector3<f32>) {
    let helper = if normal.x.abs() > 0.9 {
        Vector3::y()
    } else {
        Vector3::x()
    };

    let tangent = normal.cross(&helper).normalize();
    let bitangent = normal.cross(&tangent).normalize();

    (tangent, bitangent)
}
