use antler_id::ObjectId;
use nalgebra::{Point2, Point3, Unit, Vector3};

pub struct Hit {
    pub object_id: ObjectId,
    pub distance: f32,
    pub position: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,
    pub tangent: Unit<Vector3<f32>>,
    pub bi_tangent: Unit<Vector3<f32>>,
    pub uv: Point2<f32>,
}

impl Hit {
    #[inline]
    pub fn new(
        object_id: ObjectId,
        distance: f32,
        position: Point3<f32>,
        normal: Unit<Vector3<f32>>,
        uv: Point2<f32>,
    ) -> Self {
        let n = normal.into_inner();

        let helper = if n.x.abs() < 0.9 {
            Vector3::x_axis().into_inner()
        } else {
            Vector3::y_axis().into_inner()
        };

        let tangent = Unit::new_normalize(helper.cross(&n));
        let bi_tangent = Unit::new_normalize(n.cross(&tangent));

        Self {
            object_id,
            distance,
            position,
            normal,
            tangent,
            bi_tangent,
            uv,
        }
    }

    #[inline]
    pub fn is_interior(&self, direction: &Unit<Vector3<f32>>) -> bool {
        self.normal.dot(direction) > 0.0
    }
}
