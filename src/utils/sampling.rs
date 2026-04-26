use nalgebra::{Unit, Vector3};

pub fn hemisphere_direction(normal: Unit<Vector3<f32>>, index: usize, samples: usize) -> Unit<Vector3<f32>> {
    let n = normal.into_inner();

    let helper = if n.x.abs() < 0.9 {
        Vector3::x_axis().into_inner()
    } else {
        Vector3::y_axis().into_inner()
    };

    let tangent = n.cross(&helper).normalize();
    let bitangent = n.cross(&tangent).normalize();

    let i = index as f32;
    let count = samples.max(1) as f32;

    let u = (i + 0.5) / count;
    let v = ((i * 0.618_034) % 1.0).fract();

    let r = u.sqrt();
    let theta = std::f32::consts::TAU * v;

    let x = r * theta.cos();
    let y = r * theta.sin();
    let z = (1.0 - u).sqrt();

    Unit::new_normalize(tangent * x + bitangent * y + n * z)
}
