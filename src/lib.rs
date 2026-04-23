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
        camera::{Camera, Orthographic, Perspective},
        colour::{Gradient, Pixel, Rgb, Rgba},
        errors::{MeshLoadError, ParseHexError},
        geometry::{Aabb, Bounded, Circle, Geometry, Mesh, Quad, Ray, Sphere, Traceable, Triangle},
        id::{GeometryId, MaterialId, ObjectId, ShaderId},
        lighting::{DirectionalLight, Light, LightSample},
        material::{Material, Opaque, Scatter},
        render::{render_image, render_probe},
        shader::{Block, Checkerboard, Lambertion, Luminous, Shader},
        storage::{Grid, Image, RgbImage, RgbaImage},
        tracing::{ObjectHit, ObjectRay, Probe, WorldHit, WorldRay},
        world::{Object, Scene, World},
    };
}
