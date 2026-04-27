use nalgebra::{Point2, Point3, Unit, Vector3};

pub struct Intersection {
    pub distance: f32,
    pub position: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,
    pub tangent: Unit<Vector3<f32>>,
    pub bi_tangent: Unit<Vector3<f32>>,
    pub uv: Point2<f32>,
}

impl Intersection {
    #[must_use]
    #[inline]
    pub fn new(distance: f32, position: Point3<f32>, normal: Unit<Vector3<f32>>, uv: Point2<f32>) -> Self {
        let n = normal.into_inner();

        let helper_axis = if n.x.abs() < 0.9 {
            Vector3::x_axis().into_inner()
        } else {
            Vector3::y_axis().into_inner()
        };

        let tangent = Unit::new_normalize(helper_axis.cross(&n));
        let bi_tangent = Unit::new_normalize(n.cross(&tangent));

        Self {
            distance,
            position,
            normal,
            tangent,
            bi_tangent,
            uv,
        }
    }

    #[must_use]
    #[inline]
    pub fn is_interior(&self, direction: &Unit<Vector3<f32>>) -> bool {
        self.normal.dot(direction) > 0.0
    }
}
