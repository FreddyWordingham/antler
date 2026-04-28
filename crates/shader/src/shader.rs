use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;

use crate::{appearance::Appearance, block::Block, checkerboard::Checkerboard, luminous::Luminous, solid::Solid};

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
            Self::Block(block) => block.emitted(contact),
            Self::Checkerboard(checkerboard) => checkerboard.emitted(contact),
            Self::Luminous(luminous) => luminous.emitted(contact),
            Self::Solid(solid) => solid.emitted(contact),
        }
    }

    #[inline]
    fn albedo(&self, contact: &Contact) -> Rgb {
        match self {
            Self::Block(block) => block.albedo(contact),
            Self::Checkerboard(checkerboard) => checkerboard.albedo(contact),
            Self::Luminous(luminous) => luminous.albedo(contact),
            Self::Solid(solid) => solid.albedo(contact),
        }
    }

    #[inline]
    fn shade(&self, ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb {
        match self {
            Self::Block(block) => block.shade(ray, contact, light),
            Self::Checkerboard(checkerboard) => checkerboard.shade(ray, contact, light),
            Self::Luminous(luminous) => luminous.shade(ray, contact, light),
            Self::Solid(solid) => solid.shade(ray, contact, light),
        }
    }
}
