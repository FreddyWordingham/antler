use crate::{
    acceleration::Bvh,
    geometry::Traceable,
    id::ObjectId,
    tracing::{WorldHit, WorldRay},
    world::{Object, World},
};

pub struct Scene {
    objects: Vec<Object>,
    bvh: Bvh<ObjectId>,
}

impl Scene {
    pub fn get_object(&self, id: ObjectId) -> &Object {
        &self.objects[id.index()]
    }

    pub fn trace(&self, world: &World, world_ray: &WorldRay) -> Option<WorldHit> {
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
}
