use serde::{Deserialize, Serialize};

use crate::{
    config::{Vec3, defaults},
    errors::MeshLoadError,
    geometry::{Aabb, Circle, GeometryEnum, Mesh, Quad, Sphere},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeometryConfig {
    Aabb {
        min: Vec3,
        max: Vec3,
    },
    Sphere {
        #[serde(default)]
        centre: Vec3,
        #[serde(default = "defaults::one_f32")]
        radius: f32,
    },
    Circle {
        #[serde(default)]
        position: Vec3,
        #[serde(default = "defaults::z_axis")]
        normal: Vec3,
        #[serde(default = "defaults::one_f32")]
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

impl GeometryConfig {
    pub fn build(self) -> Result<GeometryEnum, MeshLoadError> {
        match self {
            GeometryConfig::Aabb { min, max } => Ok(Aabb::new(min.into(), max.into()).into()),
            GeometryConfig::Sphere { centre, radius } => Ok(Sphere::new(centre.into(), radius).into()),
            GeometryConfig::Circle {
                position,
                normal,
                radius,
            } => Ok(Circle::new(position.into(), normal.into(), radius).into()),
            GeometryConfig::Quad { position, normal, size } => {
                Ok(Quad::new(position.into(), normal.into(), size).into())
            }
            GeometryConfig::Mesh { path } => Ok(Mesh::load(&path)?.into()),
        }
    }
}
