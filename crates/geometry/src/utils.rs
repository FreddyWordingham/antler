use std::f32::consts::PI;

use nalgebra::{Point3, Unit, Vector3};
use rand::{Rng, RngExt};

use crate::config::RAY_BIAS;

#[must_use]
#[inline]
pub fn offset_origin(position: Point3<f32>, normal: Unit<Vector3<f32>>, direction: Unit<Vector3<f32>>) -> Point3<f32> {
    let scale = position.coords.abs().max().max(1.0);
    let bias = RAY_BIAS * scale;

    if direction.dot(&normal) >= 0.0 {
        position + *normal * bias
    } else {
        position - *normal * bias
    }
}

#[must_use]
#[inline]
pub fn tangent_frame(normal: Unit<Vector3<f32>>) -> (Unit<Vector3<f32>>, Unit<Vector3<f32>>) {
    let helper = if normal.x.abs() > 0.9 {
        Vector3::y()
    } else {
        Vector3::x()
    };

    let tangent = Unit::new_normalize(normal.cross(&helper));
    let bi_tangent = Unit::new_normalize(normal.cross(&tangent));

    (tangent, bi_tangent)
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

    let (tangent, bi_tangent) = tangent_frame(normal);

    Unit::new_normalize(*tangent * x + *bi_tangent * y + *normal * z)
}

#[inline]
pub fn hemisphere_direction<R: Rng>(
    rng: &mut R,
    normal: Unit<Vector3<f32>>,
    tangent: Unit<Vector3<f32>>,
    bi_tangent: Unit<Vector3<f32>>,
) -> Unit<Vector3<f32>> {
    let u: f32 = rng.random();
    let v: f32 = rng.random();

    let r = u.sqrt();
    let theta = std::f32::consts::TAU * v;

    Unit::new_unchecked(*tangent * (r * theta.cos()) + *bi_tangent * (r * theta.sin()) + *normal * (1.0 - u).sqrt())
}
