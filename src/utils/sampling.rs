use nalgebra::{Unit, Vector3};
use rand::{Rng, RngExt};

pub fn hemisphere_direction(normal: Unit<Vector3<f32>>, rng: &mut impl Rng) -> Unit<Vector3<f32>> {
    let u: f32 = rng.random();
    let v: f32 = rng.random();

    let r = u.sqrt();
    let theta = std::f32::consts::TAU * v;

    let n = normal.into_inner();

    let helper = if n.x.abs() < 0.9 {
        Vector3::x_axis().into_inner()
    } else {
        Vector3::y_axis().into_inner()
    };

    let tangent = n.cross(&helper).normalize();
    let bitangent = n.cross(&tangent).normalize();

    Unit::new_normalize(tangent * (r * theta.cos()) + bitangent * (r * theta.sin()) + n * (1.0 - u).sqrt())
}

pub fn cone_direction(axis: Unit<Vector3<f32>>, angle: f32, rng: &mut impl Rng) -> Unit<Vector3<f32>> {
    let u: f32 = rng.random();
    let v: f32 = rng.random();

    let cos_max = angle.cos();
    let cos_theta = 1.0 - u * (1.0 - cos_max);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
    let phi = std::f32::consts::TAU * v;

    let n = axis.into_inner();

    let helper = if n.x.abs() < 0.9 {
        Vector3::x_axis().into_inner()
    } else {
        Vector3::y_axis().into_inner()
    };

    let tangent = n.cross(&helper).normalize();
    let bitangent = n.cross(&tangent).normalize();

    Unit::new_normalize(tangent * (sin_theta * phi.cos()) + bitangent * (sin_theta * phi.sin()) + n * cos_theta)
}
