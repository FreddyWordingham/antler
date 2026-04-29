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
        #[serde(default = "default_start_position")]
        a: Vec3,
        #[serde(default = "default_end_position")]
        b: Vec3,
        #[serde(default = "default_radius")]
        radius: f32,
    },
    Circle {
        #[serde(default = "default_centre")]
        centre: Vec3,
        #[serde(default = "default_normal")]
        normal: Vec3,
        #[serde(default = "default_radius")]
        radius: f32,
    },
    Mesh {
        path: PathBuf,
    },
    Quad {
        #[serde(default = "default_centre")]
        centre: Vec3,
        #[serde(default = "default_normal")]
        normal: Vec3,
        #[serde(default = "default_size")]
        size: Vec2,
    },
    Sphere {
        #[serde(default = "default_centre")]
        centre: Vec3,
        #[serde(default = "default_radius")]
        radius: f32,
    },
    Torus {
        #[serde(default = "default_centre")]
        centre: Vec3,
        #[serde(default = "default_major_radius")]
        major_radius: f32,
        #[serde(default = "default_minor_radius")]
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
            Self::Circle { centre, normal, radius } => Circle::new(centre.into(), normal.into(), radius).into(),
            Self::Mesh { path } => Mesh::load(path).unwrap().into(),
            Self::Quad { centre, normal, size } => Quad::new(centre.into(), normal.into(), size.into()).into(),
            Self::Sphere { centre, radius } => Sphere::new(centre.into(), radius).into(),
            Self::Torus {
                centre,
                major_radius,
                minor_radius,
            } => Torus::new(centre.into(), major_radius, minor_radius).into(),
            Self::Triangle { vertices, normals, uvs } => Triangle::new(
                vertices.map(|v| v.into()),
                normals.map(|n| n.map(|v| v.into())),
                uvs.map(|u| u.map(|v| v.into())),
            )
            .into(),
        }
    }
}

fn default_centre() -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

fn default_start_position() -> Vec3 {
    Vec3::new(0.0, 0.0, 1.0)
}

fn default_end_position() -> Vec3 {
    Vec3::new(0.0, 0.0, -1.0)
}

fn default_normal() -> Vec3 {
    Vec3::new(0.0, 0.0, 1.0)
}

fn default_radius() -> f32 {
    1.0
}

fn default_major_radius() -> f32 {
    1.0
}

fn default_minor_radius() -> f32 {
    0.25
}

fn default_size() -> Vec2 {
    Vec2::new(1.0, 1.0)
}
