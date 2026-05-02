use std::{fmt::Debug, path::Path};

use nalgebra::{Point2, Point3, Unit, Vector3};
use rand::{Rng, RngExt};
use tobj::{LoadOptions, load_obj};

use crate::{
    aabb::Aabb, bounded::Bounded, bvh::Bvh, contact::Contact, errors::MeshLoadError, ray::Ray, sample::Sample,
    sampleable::Sampleable, traceable::Traceable, triangle::Triangle,
};

pub struct Mesh {
    triangles: Vec<Triangle>,
    bvh: Bvh<usize>,
    total_area: f32,
    area_cdf: Vec<f32>,
}

impl Mesh {
    #[must_use]
    pub fn new(triangles: Vec<Triangle>) -> Self {
        assert!(!triangles.is_empty(), "Cannot build a mesh with no triangles.");

        let bvh = Bvh::new(
            triangles
                .iter()
                .enumerate()
                .map(|(index, triangle)| (triangle.bounds(), index))
                .collect(),
        );

        let mut total_area = 0.0;
        let mut area_cdf = Vec::with_capacity(triangles.len());
        for triangle in &triangles {
            total_area += triangle.area();
            area_cdf.push(total_area);
        }

        Self {
            triangles,
            bvh,
            total_area,
            area_cdf,
        }
    }

    #[must_use]
    #[inline]
    pub fn triangle(&self, index: usize) -> &Triangle {
        &self.triangles[index]
    }

    pub fn load<P: AsRef<Path> + Debug>(path: P) -> Result<Self, MeshLoadError> {
        let (models, _materials) = load_obj(
            path,
            &LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )?;

        let mut triangles = Vec::new();

        for model in models {
            let mesh = model.mesh;

            if mesh.indices.len() % 3 != 0 {
                return Err(MeshLoadError::InvalidIndexCount {
                    model_name: model.name,
                    count: mesh.indices.len(),
                });
            }

            for face in mesh.indices.chunks_exact(3) {
                let i0 = face[0] as usize;
                let i1 = face[1] as usize;
                let i2 = face[2] as usize;

                let vertices = [
                    read_position(&mesh.positions, i0).ok_or_else(|| MeshLoadError::MissingPositionIndex {
                        model_name: model.name.clone(),
                        index: i0,
                    })?,
                    read_position(&mesh.positions, i1).ok_or_else(|| MeshLoadError::MissingPositionIndex {
                        model_name: model.name.clone(),
                        index: i1,
                    })?,
                    read_position(&mesh.positions, i2).ok_or_else(|| MeshLoadError::MissingPositionIndex {
                        model_name: model.name.clone(),
                        index: i2,
                    })?,
                ];

                let normals = if mesh.normals.is_empty() {
                    None
                } else {
                    Some([
                        read_normal(&mesh.normals, i0).ok_or_else(|| MeshLoadError::MissingNormalIndex {
                            model_name: model.name.clone(),
                            index: i0,
                        })?,
                        read_normal(&mesh.normals, i1).ok_or_else(|| MeshLoadError::MissingNormalIndex {
                            model_name: model.name.clone(),
                            index: i1,
                        })?,
                        read_normal(&mesh.normals, i2).ok_or_else(|| MeshLoadError::MissingNormalIndex {
                            model_name: model.name.clone(),
                            index: i2,
                        })?,
                    ])
                };

                let uvs = if mesh.texcoords.is_empty() {
                    None
                } else {
                    Some([
                        read_uv(&mesh.texcoords, i0).ok_or_else(|| MeshLoadError::MissingTexcoordIndex {
                            model_name: model.name.clone(),
                            index: i0,
                        })?,
                        read_uv(&mesh.texcoords, i1).ok_or_else(|| MeshLoadError::MissingTexcoordIndex {
                            model_name: model.name.clone(),
                            index: i1,
                        })?,
                        read_uv(&mesh.texcoords, i2).ok_or_else(|| MeshLoadError::MissingTexcoordIndex {
                            model_name: model.name.clone(),
                            index: i2,
                        })?,
                    ])
                };

                triangles.push(Triangle::new(vertices, normals, uvs));
            }
        }

        if triangles.is_empty() {
            return Err(MeshLoadError::EmptyMesh);
        }

        Ok(Self::new(triangles))
    }
}

impl Bounded for Mesh {
    #[inline]
    fn bounds(&self) -> Aabb {
        Aabb::union(self.triangles.iter().map(Bounded::bounds))
    }
}

impl Traceable for Mesh {
    #[inline]
    fn hit(&self, ray: &Ray, max_distance: f32) -> bool {
        let mut limit = max_distance;

        self.bvh.any_with_limit(ray, &mut limit, |triangle_index, limit| {
            self.triangle(triangle_index).hit(ray, *limit)
        })
    }

    #[inline]
    fn distance(&self, ray: &Ray, max_distance: f32) -> Option<f32> {
        let mut best_distance = max_distance;

        self.bvh
            .nearest_with_max(ray, &mut best_distance, |triangle_index, best_distance| {
                if let Some(distance) = self.triangle(triangle_index).distance(ray, *best_distance) {
                    *best_distance = distance;
                }

                true
            });

        (best_distance < max_distance).then_some(best_distance)
    }

    #[inline]
    fn intersection(&self, ray: &Ray, max_distance: f32) -> Option<Contact> {
        let mut nearest = None;
        let mut best_distance = max_distance;

        self.bvh
            .nearest_with_max(ray, &mut best_distance, |triangle_index, best_distance| {
                let Some(contact) = self.triangle(triangle_index).intersection(ray, *best_distance) else {
                    return true;
                };

                *best_distance = contact.distance;
                nearest = Some(contact);

                true
            });

        nearest
    }
}

impl Sampleable for Mesh {
    #[inline]
    fn area(&self) -> f32 {
        self.total_area
    }

    #[inline]
    fn sample<R: Rng>(&self, rng: &mut R) -> Sample {
        let target = rng.random::<f32>() * self.total_area;

        let index = self
            .area_cdf
            .partition_point(|&area| area < target)
            .min(self.triangles.len() - 1);

        self.triangles[index].sample(rng)
    }
}

fn read_position(data: &[f32], index: usize) -> Option<Point3<f32>> {
    let base = index * 3;
    Some(Point3::new(
        *data.get(base)?,
        *data.get(base + 1)?,
        *data.get(base + 2)?,
    ))
}

fn read_normal(data: &[f32], index: usize) -> Option<Unit<Vector3<f32>>> {
    let base = index * 3;
    let normal = Vector3::new(*data.get(base)?, *data.get(base + 1)?, *data.get(base + 2)?);

    if normal.norm_squared() <= 1.0e-12 {
        None
    } else {
        Some(Unit::new_normalize(normal))
    }
}

fn read_uv(data: &[f32], index: usize) -> Option<Point2<f32>> {
    let base = index * 2;
    Some(Point2::new(*data.get(base)?, *data.get(base + 1)?))
}
