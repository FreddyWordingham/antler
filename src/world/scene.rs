use std::f32::INFINITY;

use crate::{
    acceleration::Bvh,
    geometry::{Bounded, Traceable},
    id::ObjectId,
    tracing::{WorldHit, WorldRay},
    world::{Object, World},
};

pub struct Scene {
    objects: Vec<Object>,
    bvh: Option<Bvh<ObjectId>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bvh: None,
        }
    }

    #[inline]
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
        self.bvh = None;
    }

    #[inline]
    pub fn get_object(&self, id: ObjectId) -> &Object {
        &self.objects[id.index()]
    }

    pub fn trace(&self, world: &World, world_ray: &WorldRay) -> Option<WorldHit> {
        let bvh = self.bvh.as_ref().expect("Must build BVH before tracing.");

        let mut nearest = None;
        let mut best_distance = INFINITY;

        bvh.trace_with_best(world_ray, &mut best_distance, |object_id, best_distance| {
            let object = self.get_object(object_id);
            let object_ray = world_ray.to_object_space(&object.inv_transform);

            let geometry = world.get_geometry(object.geometry_id);
            let Some(object_hit) = geometry.trace(&object_ray) else {
                return true;
            };

            let world_hit = object_hit.to_world_space(&object.transform, world_ray.origin, object_id);

            if world_hit.distance < *best_distance {
                *best_distance = world_hit.distance;
                nearest = Some(world_hit);
            }

            true
        });

        nearest
    }

    pub fn occluded(&self, world: &World, world_ray: &WorldRay, max_distance: f32) -> bool {
        let bvh = self.bvh.as_ref().expect("Must build BVH before tracing.");

        bvh.trace_any_filtered(world_ray, max_distance, |object_id| {
            let object = self.get_object(object_id);
            let object_ray = world_ray.to_object_space(&object.inv_transform);

            let geometry = world.get_geometry(object.geometry_id);
            match geometry.trace(&object_ray) {
                Some(object_hit) => {
                    let world_hit = object_hit.to_world_space(&object.transform, world_ray.origin, object_id);
                    world_hit.distance < max_distance
                }

                None => false,
            }
        })
    }

    pub fn build(&mut self, world: &World) {
        let items = self
            .objects
            .iter()
            .enumerate()
            .map(|(index, object)| {
                let bounds = world
                    .get_geometry(object.geometry_id)
                    .bounds()
                    .transform(&object.transform);

                (bounds, ObjectId::new(index))
            })
            .collect();

        self.bvh = Some(Bvh::new(items));
    }
}
