use std::path::PathBuf;

use antler_geometry::{Aabb, Capsule, Circle, Geometry, Mesh, Quad, Sphere, Torus, Triangle};
use serde::{Deserialize, Serialize};

use crate::{vec2::Vec2, vec3::Vec3};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum GeometryConfig {
    Aabb {
        min: Vec3,
        max: Vec3,
    },
    Capsule {
        a: Vec3,
        b: Vec3,
        radius: f32,
    },
    Circle {
        position: Vec3,
        normal: Vec3,
        radius: f32,
    },
    Mesh {
        path: PathBuf,
    },
    Quad {
        position: Vec3,
        normal: Vec3,
        size: Vec2,
    },
    Sphere {
        center: Vec3,
        radius: f32,
    },
    Torus {
        center: Vec3,
        major_radius: f32,
        minor_radius: f32,
    },
    Triangle {
        vertices: [Vec3; 3],
        normals: Option<[Vec3; 3]>,
        uvs: Option<[Vec2; 3]>,
    },
}

impl GeometryConfig {
    pub fn build(self) -> Geometry {
        match self {
            Self::Aabb { min, max } => Aabb::new(min.into(), max.into()).into(),
            Self::Capsule { a, b, radius } => Capsule::new(a.into(), b.into(), radius).into(),
            Self::Circle {
                position,
                normal,
                radius,
            } => Circle::new(position.into(), normal.into(), radius).into(),
            Self::Mesh { path } => Mesh::load(path).unwrap().into(),
            Self::Quad { position, normal, size } => Quad::new(position.into(), normal.into(), size.into()).into(),
            Self::Sphere { center, radius } => Sphere::new(center.into(), radius).into(),
            Self::Torus {
                center,
                major_radius,
                minor_radius,
            } => Torus::new(center.into(), major_radius, minor_radius).into(),
            Self::Triangle { vertices, normals, uvs } => Triangle::new(
                vertices.map(|v| v.into()),
                normals.map(|n| n.map(|v| v.into())),
                uvs.map(|u| u.map(|v| v.into())),
            )
            .into(),
        }
    }
}
