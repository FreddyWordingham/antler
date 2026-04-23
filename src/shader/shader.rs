use crate::{
    colour::Rgb,
    lighting::LightSample,
    shader::{Block, Checkerboard, Lambertion, Luminous},
    tracing::{WorldHit, WorldRay},
};

pub trait Shader {
    fn emitted(&self, hit: &WorldHit) -> Rgb;
    fn shade(&self, ray: &WorldRay, hit: &WorldHit, light: &LightSample) -> Rgb;
}

macro_rules! define_shader_enum {
    ($name:ident: $($ty:ident),* $(,)?) => {
        pub enum $name {
            $($ty($ty),)*
        }

        impl Shader for $name {
            fn emitted(&self, hit: &WorldHit) -> Rgb {
                match self {
                    $(Self::$ty(inner) => inner.emitted(hit),)*
                }
            }

            fn shade(&self, ray: &WorldRay, hit: &WorldHit, light: &LightSample) -> Rgb {
                match self {
                    $(Self::$ty(inner) => inner.shade(ray, hit, light),)*
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

define_shader_enum!(ShaderEnum: Block, Lambertion, Luminous, Checkerboard);
