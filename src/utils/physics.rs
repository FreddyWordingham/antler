use nalgebra::Vector3;

#[inline]
pub fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(&n) * n
}

#[inline]
pub fn refract(uv: Vector3<f32>, n: Vector3<f32>, eta: f32, cos_theta: f32) -> Vector3<f32> {
    let r_out_perp = eta * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
