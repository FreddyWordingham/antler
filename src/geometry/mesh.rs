use std::f32::INFINITY;

use crate::{
    acceleration::Bvh,
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
}

impl Bounded for Mesh {
    fn bounds(&self) -> Aabb {
        Aabb::union(self.triangles.iter().map(|triangle| triangle.bounds()))
    }
}

impl Traceable for Mesh {
    fn trace(&self, ray: &ObjectRay) -> Option<ObjectHit> {
        let mut nearest = None;
        let mut best_distance = INFINITY;

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
