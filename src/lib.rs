mod acceleration;
mod camera;
mod colour;
mod errors;
mod geometry;
mod id;
mod lighting;
mod material;
mod render;
mod shader;
mod storage;
mod tracing;
mod world;

pub mod prelude {
    pub use crate::{
        acceleration::Bvh,
        camera::{Camera, CameraEnum, Orthographic, Perspective},
        colour::{Gradient, Pixel, Rgb, Rgba},
        errors::ParseHexError,
        geometry::{Aabb, Bounded, Geometry, GeometryEnum, Ray, Sphere, Traceable},
        id::{GeometryId, MaterialId, ObjectId, ShaderId},
        lighting::{DirectionalLight, Light, LightEnum, LightSample},
        material::{Material, MaterialEnum, Opaque, Scatter},
        render::render,
        shader::{Block, Shader, ShaderEnum},
        storage::{Grid, Image, RgbImage, RgbaImage},
        tracing::{ObjectHit, ObjectRay, Probe, WorldHit, WorldRay},
        world::{Object, Scene, World},
    };
}
