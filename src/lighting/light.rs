use rand::Rng;

use crate::{
    lighting::{DirectionalLight, LightSample},
    tracing::WorldHit,
};

pub trait Light {
    fn samples(&self, hit: &WorldHit, rng: &mut impl Rng) -> Vec<LightSample>;
}

macro_rules! define_light_enum {
    ($name:ident: $($ty:ident),* $(,)?) => {
        pub enum $name {
            $($ty($ty),)*
        }

        impl Light for $name {
            fn samples(&self, hit: &WorldHit, rng: &mut impl Rng) -> Vec<LightSample> {
                match self {
                    $(Self::$ty(inner) => inner.samples(hit, rng),)*
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
