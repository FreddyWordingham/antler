use nalgebra::{Point2, Point3, Similarity3, Unit, Vector3};

use crate::{id::ObjectId, tracing::WorldHit};

pub struct ObjectHit {
    pub distance: f32,
    pub position: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,
    pub uv: Point2<f32>,
}

impl ObjectHit {
    #[inline]
    pub fn is_interior(&self, direction: &Unit<Vector3<f32>>) -> bool {
        self.normal.dot(direction) > 0.0
    }

    #[inline]
    pub fn to_world_space(
        &self,
        transform: &Similarity3<f32>,
        world_ray_origin: Point3<f32>,
        object_id: ObjectId,
    ) -> WorldHit {
        let world_position = transform.transform_point(&self.position);

        WorldHit {
            object_id,
            distance: (world_position - world_ray_origin).norm(),
            position: world_position,
            normal: Unit::new_normalize(transform.transform_vector(&self.normal)),
            uv: self.uv,
        }
    }
}
