use crate::{
    lighting::{DirectionalLight, LightSample},
    tracing::WorldHit,
};

pub trait Light {
    fn sample(&self, hit: &WorldHit) -> LightSample;
}

macro_rules! define_light_enum {
    ($name:ident: $($ty:ident),* $(,)?) => {
        pub enum $name {
            $($ty($ty),)*
        }

        impl Light for $name {
            fn sample(&self, hit: &WorldHit) -> LightSample {
                match self {
                    $(Self::$ty(inner) => inner.sample(hit),)*
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

define_light_enum!(LightEnum: DirectionalLight);
