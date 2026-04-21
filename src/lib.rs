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
        camera::Camera,
        colour::{Colour, Gradient},
        geometry::{Aabb, Bounded, Geometry, Ray, Sphere, Traceable},
        id::{GeometryId, MaterialId, ObjectId, ShaderId},
        material::{Material, Scatter},
        render::render,
        shader::Shader,
        storage::{Grid, Image},
        tracing::Photon,
        world::{Object, Scene, World},
    };
}
