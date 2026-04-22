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

        let mut nearest: Option<WorldHit> = None;

        for object_id in bvh.trace(world_ray) {
            let object = self.get_object(object_id);
            let object_ray = world_ray.to_object_space(&object.inv_transform);

            let geometry = world.get_geometry(object.geometry_id);
            let Some(object_hit) = geometry.trace(&object_ray) else {
                continue;
            };

            let world_hit = object_hit.to_world_space(&object.transform, world_ray.origin, object_id);

            match &nearest {
                Some(nearest_hit) if nearest_hit.distance <= world_hit.distance => {}
                _ => nearest = Some(world_hit),
            }
        }

        nearest
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
