use nalgebra::{Point3, RealField, Rotation3, Unit, Vector3, distance};
use std::fmt::Debug;

use crate::geometry::Ray;

/// Generates sampling rays to form an image.
#[derive(Debug, Clone)]
pub struct Camera<T: RealField> {
    /// Observation position.
    position: Point3<T>,
    /// View target.
    look_at: Point3<T>,
    /// Horizontal field of view (radians).
    field_of_view: T,
    /// Resolution of the image in pixels.
    resolution: [usize; 2],
}

impl<T: RealField> Camera<T> {
    /// Constructs a new `Camera`.
    pub fn new(position: Point3<T>, look_at: Point3<T>, field_of_view: T, resolution: [usize; 2]) -> Self {
        debug_assert!(
            distance(&position, &look_at) > T::zero(),
            "Camera position and look-at point must be distinct"
        );
        debug_assert!(field_of_view > T::zero(), "Field of view must be positive");
        debug_assert!(resolution[0] > 0, "Resolution height must be positive");
        debug_assert!(resolution[1] > 0, "Resolution width must be positive");

        Self {
            position,
            look_at,
            field_of_view,
            resolution,
        }
    }

    pub fn generate_ray(&self, pixel_index: [usize; 2]) -> Ray<T> {
        debug_assert!(pixel_index[0] < self.resolution[0], "Row index out of bounds");
        debug_assert!(pixel_index[1] < self.resolution[1], "Column index out of bounds");

        let height = T::from_usize(self.resolution[0]).unwrap();
        let width = T::from_usize(self.resolution[1]).unwrap();

        // Normalize to [-0.5, 0.5] range
        let d_row = (T::from_usize(pixel_index[0]).unwrap() / height.clone()) - T::from_f32(0.5).unwrap();
        let d_col = (T::from_usize(pixel_index[1]).unwrap() / width.clone()) - T::from_f32(0.5).unwrap();

        let aspect_ratio = width / height;
        let half_fov = self.field_of_view.clone() * T::from_f32(0.5).unwrap();

        let d_theta = -d_col * half_fov.clone();
        let d_phi = -d_row * (half_fov / aspect_ratio);

        let forward = Unit::new_normalize(&self.look_at - &self.position);
        let right = Unit::new_normalize(forward.cross(&Vector3::z()));
        let up = Unit::new_normalize(right.cross(&forward));

        let vertical_rotation = Rotation3::from_axis_angle(&right, d_phi);
        let lateral_rotation = Rotation3::from_axis_angle(&up, d_theta);

        let direction = lateral_rotation * vertical_rotation * forward;
        Ray::new(self.position.clone(), direction)
    }
}
