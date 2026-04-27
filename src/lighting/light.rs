use rand::Rng;

use crate::{
    lighting::{DirectionalLight, LightSample},
    tracing::WorldHit,
};

pub trait Light {
    fn for_each_sample(&self, hit: &WorldHit, rng: &mut impl Rng, f: impl FnMut(LightSample));
}

macro_rules! define_light_enum {
    ($name:ident: $($ty:ident),* $(,)?) => {
        pub enum $name {
            $($ty($ty),)*
        }

        impl Light for $name {
            fn for_each_sample(
                &self,
                hit: &WorldHit,
                rng: &mut impl Rng,
                f: impl FnMut(LightSample),
            ) {
                match self {
                    $(Self::$ty(inner) => inner.for_each_sample(hit, rng, f),)*
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
