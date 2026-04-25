use crate::{
    material::{Mirror, Opaque, Scatter},
    tracing::{Probe, WorldHit},
};

pub trait Material {
    fn scatter(&self, probe: &Probe, hit: &WorldHit) -> Scatter;
}

macro_rules! define_material_enum {
    ($name:ident: $($ty:ident),* $(,)?) => {
        pub enum $name {
            $($ty($ty),)*
        }

        impl Material for $name {
            fn scatter(&self, probe: &Probe, hit: &WorldHit) -> Scatter {
                match self {
                    $(Self::$ty(inner) => inner.scatter(probe, hit),)*
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

define_material_enum!(MaterialEnum: Mirror, Opaque);
