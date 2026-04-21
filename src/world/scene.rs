use crate::{acceleration::Bvh, geometry::Ray, id::ObjectId, tracing::Hit, world::Object};

pub struct Scene {
    objects: Vec<Object>,
    bvh: Bvh,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<(ObjectId, Hit)> {
        todo!()
    }

    pub fn get_object(&self, id: ObjectId) -> &Object {
        todo!()
    }
}
