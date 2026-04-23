use nalgebra::Point2;

use crate::{
    camera::{Orthographic, Perspective},
    tracing::Probe,
};

pub trait Camera {
    fn emit(&self, uv: Point2<f32>) -> Probe;
}

macro_rules! define_camera_enum {
    ($name:ident: $($ty:ident),* $(,)?) => {
        pub enum $name {
            $($ty($ty),)*
        }

        impl Camera for $name {
            fn emit(&self, uv: Point2<f32>) -> Probe {
                match self {
                    $(Self::$ty(inner) => inner.emit(uv),)*
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

define_camera_enum!(CameraEnum: Orthographic, Perspective);
