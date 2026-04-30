use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;

use crate::{
    appearance::Appearance, block::Block, checkerboard::Checkerboard, iridescent::Iridescent, luminous::Luminous,
    normal::Normal, solid::Solid,
};

pub enum Shader {
    Block(Block),
    Checkerboard(Checkerboard),
    Iridescent(Iridescent),
    Luminous(Luminous),
    Normal(Normal),
    Solid(Solid),
}

impl Appearance for Shader {
    #[inline]
    fn emitted(&self, contact: &Contact) -> Rgb {
        match self {
            Self::Block(block) => block.emitted(contact),
            Self::Checkerboard(checkerboard) => checkerboard.emitted(contact),
            Self::Iridescent(iridescent) => iridescent.emitted(contact),
            Self::Luminous(luminous) => luminous.emitted(contact),
            Self::Normal(normal) => normal.emitted(contact),
            Self::Solid(solid) => solid.emitted(contact),
        }
    }

    #[inline]
    fn albedo(&self, contact: &Contact) -> Rgb {
        match self {
            Self::Block(block) => block.albedo(contact),
            Self::Checkerboard(checkerboard) => checkerboard.albedo(contact),
            Self::Iridescent(iridescent) => iridescent.albedo(contact),
            Self::Luminous(luminous) => luminous.albedo(contact),
            Self::Normal(normal) => normal.albedo(contact),
            Self::Solid(solid) => solid.albedo(contact),
        }
    }

    #[inline]
    fn shade(&self, ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb {
        match self {
            Self::Block(block) => block.shade(ray, contact, light),
            Self::Checkerboard(checkerboard) => checkerboard.shade(ray, contact, light),
            Self::Iridescent(iridescent) => iridescent.shade(ray, contact, light),
            Self::Luminous(luminous) => luminous.shade(ray, contact, light),
            Self::Normal(normal) => normal.shade(ray, contact, light),
            Self::Solid(solid) => solid.shade(ray, contact, light),
        }
    }
}

impl From<Block> for Shader {
    #[inline]
    fn from(val: Block) -> Self {
        Self::Block(val)
    }
}

impl From<Checkerboard> for Shader {
    #[inline]
    fn from(val: Checkerboard) -> Self {
        Self::Checkerboard(val)
    }
}

impl From<Iridescent> for Shader {
    #[inline]
    fn from(val: Iridescent) -> Self {
        Self::Iridescent(val)
    }
}

impl From<Luminous> for Shader {
    #[inline]
    fn from(val: Luminous) -> Self {
        Self::Luminous(val)
    }
}

impl From<Normal> for Shader {
    #[inline]
    fn from(val: Normal) -> Self {
        Self::Normal(val)
    }
}

impl From<Solid> for Shader {
    #[inline]
    fn from(val: Solid) -> Self {
        Self::Solid(val)
    }
}
