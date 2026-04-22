use crate::{
    acceleration::Bvh,
    geometry::Traceable,
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
        // if self.bvh.is_none() {
        //     panic!("Must (re)build BVH before tracing!");
        // }

        let mut nearest: Option<WorldHit> = None;

        for (index, object) in self.objects.iter().enumerate() {
            let object_ray = world_ray.to_object_space(&object.inv_transform);

            let geometry = world.get_geometry(object.geometry_id);
            let Some(object_hit) = geometry.trace(&object_ray) else {
                continue;
            };

            let world_hit = object_hit.to_world_space(&object.transform, world_ray.origin, ObjectId::new(index));
            match &nearest {
                Some(nearest_hit) if nearest_hit.distance <= world_hit.distance => {}
                _ => nearest = Some(world_hit),
            }
        }

        nearest
    }

    pub fn build_bvh(&mut self, _world: &World) {
        // TODO: Build BVH from objects and their geometries
    }
}
