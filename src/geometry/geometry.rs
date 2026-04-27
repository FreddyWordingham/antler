use crate::{
    geometry::{Aabb, Bounded, Circle, Mesh, Quad, Sphere, Traceable, Triangle},
    tracing::{ObjectHit, ObjectRay},
};

pub trait Geometry: Bounded + Traceable {}
impl<T: Bounded + Traceable> Geometry for T {}

macro_rules! define_geometry_enum {
    ($name:ident: $($ty:ident),* $(,)?) => {
        pub enum $name {
            $($ty($ty),)*
        }

        impl Bounded for $name {
            fn bounds(&self) -> Aabb {
                match self {
                    $(Self::$ty(inner) => inner.bounds(),)*
                }
            }
        }

        impl Traceable for $name {
            fn trace(&self, ray: &ObjectRay) -> Option<ObjectHit> {
                match self {
                    $(Self::$ty(inner) => inner.trace(ray),)*
                }
            }

            fn trace_distance(&self, ray: &ObjectRay) -> Option<f32> {
                match self {
                    $(Self::$ty(inner) => inner.trace_distance(ray),)*
                }
            }
        }

        $(
            impl From<$ty> for $name {
                fn from(value: $ty) -> Self {
                    Self::$ty(value)
                }
            }
        )*
    };
}

define_geometry_enum!(GeometryEnum: Aabb, Sphere, Circle, Quad, Triangle, Mesh);
