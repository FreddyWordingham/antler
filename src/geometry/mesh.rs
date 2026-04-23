use std::path::Path;

use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{
    acceleration::Bvh,
    errors::MeshLoadError,
    geometry::{Aabb, Bounded, Traceable, Triangle},
    tracing::{ObjectHit, ObjectRay},
};

pub struct Mesh {
    pub triangles: Vec<Triangle>,
    bvh: Bvh<usize>,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        assert!(!triangles.is_empty(), "Cannot build a mesh with no triangles.");

        let items = triangles
            .iter()
            .enumerate()
            .map(|(index, triangle)| (triangle.bounds(), index))
            .collect();

        let bvh = Bvh::new(items);

        Self { triangles, bvh }
    }

    #[inline]
    pub fn triangle(&self, index: usize) -> &Triangle {
        &self.triangles[index]
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, MeshLoadError> {
        let (models, _materials) = tobj::load_obj(
            path.as_ref(),
            &tobj::LoadOptions {
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
    fn bounds(&self) -> Aabb {
        Aabb::union(self.triangles.iter().map(|triangle| triangle.bounds()))
    }
}

impl Traceable for Mesh {
    fn trace(&self, ray: &ObjectRay) -> Option<ObjectHit> {
        let mut nearest = None;
        let mut best_distance = f32::INFINITY;

        self.bvh
            .trace_nearest_with_max(ray, &mut best_distance, |triangle_index, best_distance| {
                let Some(hit) = self.triangle(triangle_index).trace(ray) else {
                    return true;
                };

                if hit.distance < *best_distance {
                    *best_distance = hit.distance;
                    nearest = Some(hit);
                }

                true
            });

        nearest
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
