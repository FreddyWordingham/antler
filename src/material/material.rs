use crate::{
    material::{Mirror, Opaque, Reflective, Refractive},
    tracing::{Probe, WorldHit, WorldRay},
};

pub trait Material {
    fn scatter(&self, probe: &Probe, hit: &WorldHit, emit_child: impl FnMut(f32, WorldRay)) -> f32;
}

macro_rules! define_material_enum {
    ($name:ident: $($ty:ident),* $(,)?) => {
        pub enum $name {
            $($ty($ty),)*
        }

        impl Material for $name {
            fn scatter(&self, probe: &Probe, hit: &WorldHit, emit_child: impl FnMut(f32, WorldRay)) -> f32 {
                match self {
                    $(Self::$ty(inner) => inner.scatter(probe, hit, emit_child),)*
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

define_material_enum!(MaterialEnum: Mirror, Opaque, Reflective, Refractive);
