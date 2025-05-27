//! Triangle mesh structure.

use nalgebra::{Point3, RealField, Unit, Vector3};
use std::{fs::read_to_string, path::Path, str::FromStr};

use crate::geometry::{Aabb, Bounded, Bvh, IndexedBounds, Intersection, Ray, Triangle};

const MAX_BVH_CHILDREN: usize = 4; // Maximum number of children per BVH node.
const MAX_BVH_DEPTH: usize = 8; // Maximum depth of the BVH tree.

/// Triangular face.
struct Face {
    /// Vertex position indices.
    vertex_indices: [usize; 3],
    /// Vertex normal indices.
    normal_indices: [usize; 3],
}

/// Triangle mesh.
pub struct Mesh<T: RealField> {
    /// Vertex positions.
    vertices: Vec<Point3<T>>,
    /// Vertex normals.
    normals: Vec<Unit<Vector3<T>>>,
    /// List of faces.
    faces: Vec<Face>,
    /// Bounding Volume Hierarchy.
    bvh: Bvh<T>,
}

impl<T: RealField> Mesh<T> {
    /// Load a `Mesh` from a wavefront (.obj) file.
    #[inline]
    pub fn load(path: &Path) -> Mesh<T>
    where
        T: FromStr,
    {
        let file_string = read_to_string(path).unwrap();
        Self::from_str(&file_string)
    }

    /// Construct a `Mesh` from a wavefton (.obj) string.
    #[inline]
    pub fn from_str(obj_string: &str) -> Mesh<T>
    where
        T: FromStr,
    {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut faces = Vec::new();

        for line in obj_string.lines() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            match *tokens
                .first()
                .expect("Invalid .obj data: Expected at least item in each wavefront file line")
            {
                "v" => {
                    let vertex = parse_vertex_position(&tokens[1..]);
                    vertices.push(vertex);
                }
                "vn" => {
                    let normal = parse_vertex_normal(&tokens[1..]);
                    normals.push(normal);
                }
                "f" => {
                    let face = parse_face(&tokens[1..]);
                    faces.push(face);
                }
                _ => {}
            }
        }

        let triangles = faces
            .iter()
            .map(|face| {
                Triangle::new(
                    [
                        vertices[face.vertex_indices[0]].clone(),
                        vertices[face.vertex_indices[1]].clone(),
                        vertices[face.vertex_indices[2]].clone(),
                    ],
                    [
                        normals[face.normal_indices[0]].clone(),
                        normals[face.normal_indices[1]].clone(),
                        normals[face.normal_indices[2]].clone(),
                    ],
                )
            })
            .collect::<Vec<_>>();

        Self {
            vertices,
            normals,
            faces,
            bvh: Bvh::new(&triangles, MAX_BVH_CHILDREN, MAX_BVH_DEPTH),
        }
    }

    /// Get the depth of the BVH tree.
    #[must_use]
    #[inline]
    pub fn bvh_depth(&self) -> usize {
        self.bvh.depth()
    }

    /// Build a `Triangle` from the mesh at the specified index.
    #[must_use]
    #[inline]
    pub fn triangle(&self, index: usize) -> Triangle<T> {
        let face = &self.faces[index];
        Triangle::new(
            [
                self.vertices[face.vertex_indices[0]].clone(),
                self.vertices[face.vertex_indices[1]].clone(),
                self.vertices[face.vertex_indices[2]].clone(),
            ],
            [
                self.normals[face.normal_indices[0]].clone(),
                self.normals[face.normal_indices[1]].clone(),
                self.normals[face.normal_indices[2]].clone(),
            ],
        )
    }

    /// Iterate over all the `Triangle`s of the `Mesh`.
    #[inline]
    pub fn iter_triangles(&self) -> impl Iterator<Item = Triangle<T>> + '_ {
        (0..self.faces.len()).map(|n| self.triangle(n))
    }

    /// Compute an intersection between a `Ray` and the `Mesh`.
    pub fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>> {
        // self.iter_triangles()
        //     .filter_map(|triangle| triangle.intersect(ray))
        //     .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal))

        self.bvh
            .ray_intersections(ray, self)
            .into_iter()
            .filter_map(|(n, _)| self.triangle(n).intersect(ray).map(|ix| (n, ix)))
            .min_by(|(_, a), (_, b)| a.distance.partial_cmp(&b.distance).unwrap())
            .map(|(_, ix)| ix)
    }
}

impl<T: RealField> IndexedBounds<T> for Mesh<T> {
    #[inline]
    fn indexed_aabb(&self, index: usize) -> Aabb<T> {
        self.triangle(index).aabb()
    }
}

// == Utility functions ==

/// Parse a vertex position from an .obj file string.
#[inline]
fn parse_vertex_position<T: RealField + FromStr>(coords: &[&str]) -> Point3<T> {
    assert!(coords.len() == 3, "Vertex position must have exactly 3 coordinates");
    let x = coords[0]
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Invalid x coordinate for vertex position"));
    let y = coords[1]
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Invalid y coordinate for vertex position"));
    let z = coords[2]
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Invalid z coordinate for vertex position"));
    Point3::new(x, y, z)
}

/// Parse a vertex normal from an .obj file string.
#[inline]
fn parse_vertex_normal<T: RealField + FromStr>(coords: &[&str]) -> Unit<Vector3<T>> {
    assert!(coords.len() == 3, "Vertex normal must have exactly 3 coordinates");
    let xn = coords[0]
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Invalid x coordinate for vertex normal"));
    let yn = coords[1]
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Invalid y coordinate for vertex normal"));
    let zn = coords[2]
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Invalid z coordinate for vertex normal"));
    Unit::new_normalize(Vector3::new(xn, yn, zn))
}

/// Parse a face from an .obj file string.
#[inline]
fn parse_face(tokens: &[&str]) -> Face {
    assert!(
        tokens.len() == 3,
        "Face must have exactly 3 vertex indices (must be triangular face mesh)"
    );

    let mut vertex_indices = [0; 3];
    let mut normal_indices = [0; 3];

    for (i, token) in tokens.iter().enumerate() {
        vertex_indices[i] = token
            .split('/')
            .next()
            .expect("Face must specify a vertex position index")
            .parse::<usize>()
            .expect("Invalid face vertex position index!")
            - 1;
        normal_indices[i] = token
            .split('/')
            .last()
            .expect("Face must specify a vertex normal index")
            .parse::<usize>()
            .expect("Invalid face vertex normal index")
            - 1;
    }

    Face {
        vertex_indices,
        normal_indices,
    }
}
