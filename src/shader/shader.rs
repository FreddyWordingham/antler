use crate::{
    colour::Rgb,
    lighting::LightSample,
    shader::{Block, Checkerboard, Lambertion, Luminous},
    tracing::{WorldHit, WorldRay},
};

pub trait Shader {
    fn emitted(&self, hit: &WorldHit) -> Rgb;
    fn albedo(&self, hit: &WorldHit) -> Rgb;
    fn shade(&self, hit: &WorldHit, ray: &WorldRay, light: &LightSample) -> Rgb;
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

            fn albedo(&self, hit: &WorldHit) -> Rgb {
                match self {
                    $(Self::$ty(inner) => inner.albedo(hit),)*
                }
            }

            fn shade(&self, hit: &WorldHit,ray: &WorldRay,  light: &LightSample) -> Rgb {
                match self {
                    $(Self::$ty(inner) => inner.shade(hit, ray, light),)*
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
