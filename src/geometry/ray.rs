//! Ray structure.

use nalgebra::{Point3, RealField, Rotation3, Unit, Vector3};

#[derive(Debug, Clone)]
pub struct Ray<T: RealField> {
    /// Starting location.
    pub origin: Point3<T>,
    /// Direction.
    pub direction: Unit<Vector3<T>>,
}

impl<T: RealField> Ray<T> {
    /// Construct a new `Ray` instance.
    pub const fn new(origin: Point3<T>, direction: Unit<Vector3<T>>) -> Self {
        Self { origin, direction }
    }

    /// Travel the `Ray` along its direction by a distance `t`.
    pub fn travel(&mut self, t: T) {
        self.origin += self.direction.as_ref() * t;
    }

    /// Rotate the `Ray` with a given pitch and subsequent roll manoeuver.
    #[inline]
    pub fn rotate(&mut self, pitch: T, roll: T) {
        let arbitrary_axis = if (T::one() - (&self.direction.z).abs()) >= T::from_f32(1.0e-1).unwrap() {
            Vector3::z_axis()
        } else {
            Vector3::y_axis()
        };

        let pitch_axis = Unit::new_normalize(self.direction.cross(&arbitrary_axis));
        let pitch_rot = Rotation3::from_axis_angle(&pitch_axis, pitch);

        let roll_rot = Rotation3::from_axis_angle(&self.direction, roll);

        self.direction = roll_rot * pitch_rot * &self.direction;
        self.direction.renormalize();
    }

    /// Reflect the `Ray`'s direction from a normal.
    #[inline]
    pub fn reflect(&mut self, normal: Unit<Vector3<T>>) {
        let i = self.direction.as_ref().clone();
        let n = normal.as_ref().clone();
        let two = T::from_f32(2.0).unwrap();
        let dot = i.dot(&n);
        let reflected = i - n * (two * dot);
        self.direction = Unit::new_normalize(reflected);
    }

    /// Refract the `Ray`'s direction based on Snell's law.
    /// Returns `true` if refraction was successful, `false` if total internal reflection occurred.
    #[inline]
    pub fn refract(&mut self, normal: Unit<Vector3<T>>, n1: T, n2: T) -> bool {
        debug_assert!(n1 > T::zero() && n2 > T::zero(), "Refractive indices must be positive");

        let i = self.direction.as_ref().clone();
        let n = normal.as_ref().clone();
        let eta = n1 / n2;
        let cos_i = -i.dot(&n);
        let sin2_t = eta.clone().powi(2) * (T::one() - cos_i.clone() * cos_i.clone());

        if sin2_t > T::one() {
            // Total internal reflection
            self.reflect(normal);
            return false;
        }

        let cos_t = (T::one() - sin2_t).sqrt();
        let refracted = i * eta.clone() + n * (eta * cos_i - cos_t);
        self.direction = Unit::new_normalize(refracted);
        true
    }
}
