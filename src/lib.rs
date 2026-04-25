mod acceleration;
mod camera;
mod colour;
mod config;
mod errors;
mod geometry;
mod id;
mod lighting;
mod material;
mod render;
mod shader;
mod storage;
mod tracing;
mod utils;
mod world;

pub mod prelude {
    pub use crate::{
        camera::{Orthographic, Perspective},
        colour::{Gradient, Rgb, Rgba},
        config::*,
        errors::{MeshLoadError, ParseHexError, SceneBuildError},
        geometry::{Aabb, Bounded, Circle, Geometry, Mesh, Quad, Ray, Sphere, Triangle},
        lighting::DirectionalLight,
        material::{Mirror, Opaque, Reflective, Refractive},
        render::{RenderSettings, render_image, render_image_with_progress, render_probe},
        shader::{Block, Checkerboard, Lambertion, Luminous},
        storage::{Grid, Image, RgbImage, RgbaImage},
        world::{Object, Scene, World},
    };
}
