use nalgebra::{Point2, Point3, Unit, Vector3};

use crate::{aabb::Aabb, bounded::Bounded, config::MIN_RAY_DISTANCE, contact::Contact, ray::Ray, traceable::Traceable};

const BOUNDS_PADDING: f32 = 1.0e-6;
const CONTACT_EPSILON: f32 = 1.0e-8;

pub struct Triangle {
    vertices: [Point3<f32>; 3],
    normals: Option<[Unit<Vector3<f32>>; 3]>,
    uvs: Option<[Point2<f32>; 3]>,
}

impl Triangle {
    #[must_use]
    pub const fn new(
        vertices: [Point3<f32>; 3],
        normals: Option<[Unit<Vector3<f32>>; 3]>,
        uvs: Option<[Point2<f32>; 3]>,
    ) -> Self {
        Self { vertices, normals, uvs }
    }

    #[must_use]
    #[inline]
    pub fn face_normal(&self) -> Unit<Vector3<f32>> {
        Unit::new_normalize((self.vertices[1] - self.vertices[0]).cross(&(self.vertices[2] - self.vertices[0])))
    }

    #[inline]
    fn distance_unchecked(&self, ray: &Ray) -> Option<f32> {
        self.intersect(ray).map(|(distance, _)| distance)
    }

    fn intersect(&self, ray: &Ray) -> Option<(f32, Vector3<f32>)> {
        let edge_ab = self.vertices[1] - self.vertices[0];
        let edge_ac = self.vertices[2] - self.vertices[0];

        let scale = edge_ab.norm().max(edge_ac.norm()).max(1.0);
        let eps = CONTACT_EPSILON * scale;

        let p_vec = ray.direction.cross(&edge_ac);
        let det = edge_ab.dot(&p_vec);

        if det.abs() < eps {
            return None;
        }

        let inv_det = 1.0 / det;
        let t_vec = ray.origin - self.vertices[0];

        let beta = t_vec.dot(&p_vec) * inv_det;
        if beta < -eps || beta > 1.0 + eps {
            return None;
        }

        let q_vec = t_vec.cross(&edge_ab);
        let gamma = ray.direction.dot(&q_vec) * inv_det;
        if gamma < -eps || beta + gamma > 1.0 + eps {
            return None;
        }

        let distance = edge_ac.dot(&q_vec) * inv_det;
        if distance <= eps.max(MIN_RAY_DISTANCE) {
            return None;
        }

        let alpha = 1.0 - beta - gamma;
        Some((distance, Vector3::new(alpha, beta, gamma)))
    }

    #[must_use]
    #[inline]
    fn interpolate_position(&self, bary: Vector3<f32>) -> Point3<f32> {
        Point3::from(
            self.vertices[0].coords * bary.x + self.vertices[1].coords * bary.y + self.vertices[2].coords * bary.z,
        )
    }

    #[must_use]
    #[inline]
    fn interpolate_normal(&self, bary: Vector3<f32>) -> Unit<Vector3<f32>> {
        match &self.normals {
            Some([na, nb, nc]) => {
                let normal = na.into_inner() * bary.x + nb.into_inner() * bary.y + nc.into_inner() * bary.z;

                if normal.norm_squared() > f32::EPSILON {
                    Unit::new_normalize(normal)
                } else {
                    self.face_normal()
                }
            }
            None => self.face_normal(),
        }
    }

    #[inline]
    fn interpolate_uv(&self, bary: Vector3<f32>) -> Point2<f32> {
        match self.uvs {
            Some([uva, uvb, uvc]) => Point2::from(uva.coords * bary.x + uvb.coords * bary.y + uvc.coords * bary.z),
            None => Point2::new(bary.y, bary.z),
        }
    }
}

impl Bounded for Triangle {
    fn bounds(&self) -> Aabb {
        let min = Point3::new(
            self.vertices[0].x.min(self.vertices[1].x).min(self.vertices[2].x),
            self.vertices[0].y.min(self.vertices[1].y).min(self.vertices[2].y),
            self.vertices[0].z.min(self.vertices[1].z).min(self.vertices[2].z),
        );
        let max = Point3::new(
            self.vertices[0].x.max(self.vertices[1].x).max(self.vertices[2].x),
            self.vertices[0].y.max(self.vertices[1].y).max(self.vertices[2].y),
            self.vertices[0].z.max(self.vertices[1].z).max(self.vertices[2].z),
        );

        let scale = self
            .vertices
            .iter()
            .map(|p| p.coords.abs().max())
            .fold(1.0_f32, f32::max);
        let padding = Vector3::new(BOUNDS_PADDING, BOUNDS_PADDING, BOUNDS_PADDING) * scale;

        Aabb::new(min - padding, max + padding)
    }
}

impl Traceable for Triangle {
    #[inline]
    fn hit(&self, ray: &Ray, max_distance: f32) -> bool {
        self.distance(ray, max_distance).is_some()
    }

    #[inline]
    fn distance(&self, ray: &Ray, max_distance: f32) -> Option<f32> {
        self.distance_unchecked(ray)
            .filter(|distance| *distance > MIN_RAY_DISTANCE && *distance < max_distance)
    }

    #[inline]
    fn intersection(&self, ray: &Ray, max_distance: f32) -> Option<Contact> {
        let (distance, bary) = self.intersect(ray)?;

        if distance >= max_distance {
            return None;
        }

        let position = self.interpolate_position(bary);
        let mut normal = self.interpolate_normal(bary);

        if normal.dot(&ray.direction) > 0.0 {
            normal = -normal;
        }

        Some(Contact::new(distance, position, normal, self.interpolate_uv(bary)))
    }
}
