mod acceleration;
mod camera;
mod colour;
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
        colour::{Colour, Gradient, Rgb, Rgba},
        geometry::{Aabb, Bounded, Geometry, GeometryEnum, Ray, Sphere, Traceable},
        id::{GeometryId, MaterialId, ObjectId, ShaderId},
        material::{Material, MaterialEnum, Scatter},
        render::render,
        shader::{Shader, ShaderEnum},
        storage::{Grid, Image, RgbImage, RgbaImage},
        tracing::Photon,
        world::{Object, Scene, World},
    };
}
