use nalgebra::{Point2, Point3, Similarity3, Unit, Vector3};

use crate::utils::tangent_frame;

pub struct Contact {
    pub distance: f32,
    pub position: Point3<f32>,
    pub normal: Unit<Vector3<f32>>,
    pub uv: Point2<f32>,
    tangent: Option<Unit<Vector3<f32>>>,
    bi_tangent: Option<Unit<Vector3<f32>>>,
}

impl Contact {
    #[must_use]
    #[inline]
    pub const fn new(distance: f32, position: Point3<f32>, normal: Unit<Vector3<f32>>, uv: Point2<f32>) -> Self {
        Self {
            distance,
            position,
            normal,
            tangent: None,
            bi_tangent: None,
            uv,
        }
    }

    #[must_use]
    #[inline]
    pub fn is_interior(&self, direction: &Unit<Vector3<f32>>) -> bool {
        self.normal.dot(direction) > 0.0
    }

    #[must_use]
    #[inline]
    pub fn tangent(&mut self) -> Unit<Vector3<f32>> {
        if self.tangent.is_none() {
            self.calculate_tangents();
        }
        self.tangent.unwrap()
    }

    #[must_use]
    #[inline]
    pub fn bi_tangent(&mut self) -> Unit<Vector3<f32>> {
        if self.bi_tangent.is_none() {
            self.calculate_tangents();
        }
        self.bi_tangent.unwrap()
    }

    #[must_use]
    #[inline]
    pub fn transform(&self, transform: &Similarity3<f32>, ray_origin: Point3<f32>) -> Self {
        let position = transform.transform_point(&self.position);
        let distance = (position - ray_origin).norm();
        let normal = Unit::new_normalize(transform.transform_vector(&self.normal));
        let (tangent, bi_tangent) = tangent_frame(normal);

        Self {
            distance,
            position,
            normal,
            uv: self.uv,
            tangent: Some(tangent),
            bi_tangent: Some(bi_tangent),
        }
    }

    fn calculate_tangents(&mut self) {
        if self.tangent.is_some() && self.bi_tangent.is_some() {
            return;
        }

        let n = self.normal.into_inner();

        let helper_axis = if n.x.abs() < 0.9 {
            Vector3::x_axis().into_inner()
        } else {
            Vector3::y_axis().into_inner()
        };

        let tangent = Unit::new_normalize(helper_axis.cross(&n));
        let bi_tangent = Unit::new_normalize(n.cross(&tangent));

        self.tangent = Some(tangent);
        self.bi_tangent = Some(bi_tangent);
    }
}
