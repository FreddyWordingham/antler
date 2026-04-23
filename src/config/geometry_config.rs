use serde::{Deserialize, Serialize};

use crate::{
    config::{Vec3, defaults},
    geometry::{Aabb, Circle, GeometryEnum, Mesh, Quad, Sphere},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GeometryConfig {
    Aabb {
        min: Vec3,
        max: Vec3,
    },
    Sphere {
        #[serde(default)]
        centre: Vec3,
        #[serde(default = "defaults::one")]
        radius: f32,
    },
    Circle {
        #[serde(default)]
        position: Vec3,
        #[serde(default = "defaults::z_axis")]
        normal: Vec3,
        #[serde(default = "defaults::one")]
        radius: f32,
    },
    Quad {
        #[serde(default)]
        position: Vec3,
        #[serde(default = "defaults::z_axis")]
        normal: Vec3,
        #[serde(default = "defaults::unit_square")]
        size: [f32; 2],
    },
    Mesh {
        path: String,
    },
}

impl From<GeometryConfig> for GeometryEnum {
    fn from(config: GeometryConfig) -> Self {
        match config {
            GeometryConfig::Aabb { min, max } => GeometryEnum::Aabb(Aabb::new(min.into(), max.into())),
            GeometryConfig::Sphere { centre, radius } => GeometryEnum::Sphere(Sphere::new(centre.into(), radius)),
            GeometryConfig::Circle {
                position,
                normal,
                radius,
            } => GeometryEnum::Circle(Circle::new(position.into(), normal.into(), radius)),
            GeometryConfig::Quad { position, normal, size } => {
                GeometryEnum::Quad(Quad::new(position.into(), normal.into(), size))
            }
            GeometryConfig::Mesh { path } => GeometryEnum::Mesh(Mesh::load(&path).expect("Failed to load mesh")),
        }
    }
}
