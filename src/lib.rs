mod acceleration;
mod camera;
mod colour;
mod errors;
mod geometry;
mod id;
mod material;
mod render;
mod shader;
mod storage;
mod tracing;
mod world;

pub mod prelude {
    pub use crate::{
        acceleration::Bvh,
        camera::{Camera, CameraEnum},
        colour::{Gradient, Pixel, Rgb, Rgba},
        errors::ParseHexError,
        geometry::{Aabb, Bounded, Geometry, GeometryEnum, Ray, Sphere, Traceable},
        id::{GeometryId, MaterialId, ObjectId, ShaderId},
        material::{Material, MaterialEnum, Opaque, Scatter},
        render::render,
        shader::{Shader, ShaderEnum},
        storage::{Grid, Image, RgbImage, RgbaImage},
        tracing::Probe,
        world::{Object, Scene, World},
    };
}
