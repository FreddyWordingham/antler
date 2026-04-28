use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};

use crate::{
    appearance::Appearance, block::Block, checkerboard::Checkerboard, light_sample::LightSample, luminous::Luminous,
    solid::Solid,
};

pub enum Shader {
    Block(Block),
    Checkerboard(Checkerboard),
    Luminous(Luminous),
    Solid(Solid),
}

impl Appearance for Shader {
    #[inline]
    fn emitted(&self, contact: &Contact) -> Rgb {
        match self {
            Shader::Block(block) => block.emitted(contact),
            Shader::Checkerboard(checkerboard) => checkerboard.emitted(contact),
            Shader::Luminous(luminous) => luminous.emitted(contact),
            Shader::Solid(solid) => solid.emitted(contact),
        }
    }

    #[inline]
    fn albedo(&self, contact: &Contact) -> Rgb {
        match self {
            Shader::Block(block) => block.albedo(contact),
            Shader::Checkerboard(checkerboard) => checkerboard.albedo(contact),
            Shader::Luminous(luminous) => luminous.albedo(contact),
            Shader::Solid(solid) => solid.albedo(contact),
        }
    }

    #[inline]
    fn shade(&self, ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb {
        match self {
            Shader::Block(block) => block.shade(ray, contact, light),
            Shader::Checkerboard(checkerboard) => checkerboard.shade(ray, contact, light),
            Shader::Luminous(luminous) => luminous.shade(ray, contact, light),
            Shader::Solid(solid) => solid.shade(ray, contact, light),
        }
    }
}
