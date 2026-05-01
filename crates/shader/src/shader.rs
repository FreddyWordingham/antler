use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;

use crate::{
    angular::Angular, appearance::Appearance, block::Block, checkerboard::Checkerboard, gradient::Gradient,
    iridescent::Iridescent, luminous::Luminous, normal::Normal, solid::Solid, textured::Textured, wireframe::Wireframe,
};

pub enum Shader {
    Angular(Angular),
    Block(Block),
    Checkerboard(Checkerboard),
    Gradient(Gradient),
    Iridescent(Iridescent),
    Luminous(Luminous),
    Normal(Normal),
    Solid(Solid),
    Textured(Textured),
    Wireframe(Wireframe),
}

impl Appearance for Shader {
    #[inline]
    fn emitted(&self, contact: &Contact) -> Rgb {
        match self {
            Self::Angular(angular) => angular.emitted(contact),
            Self::Block(block) => block.emitted(contact),
            Self::Checkerboard(checkerboard) => checkerboard.emitted(contact),
            Self::Gradient(gradient) => gradient.emitted(contact),
            Self::Iridescent(iridescent) => iridescent.emitted(contact),
            Self::Luminous(luminous) => luminous.emitted(contact),
            Self::Normal(normal) => normal.emitted(contact),
            Self::Solid(solid) => solid.emitted(contact),
            Self::Textured(textured) => textured.emitted(contact),
            Self::Wireframe(wireframe) => wireframe.emitted(contact),
        }
    }

    #[inline]
    fn shade(&self, ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb {
        match self {
            Self::Angular(angular) => angular.shade(ray, contact, light),
            Self::Block(block) => block.shade(ray, contact, light),
            Self::Checkerboard(checkerboard) => checkerboard.shade(ray, contact, light),
            Self::Gradient(gradient) => gradient.shade(ray, contact, light),
            Self::Iridescent(iridescent) => iridescent.shade(ray, contact, light),
            Self::Luminous(luminous) => luminous.shade(ray, contact, light),
            Self::Normal(normal) => normal.shade(ray, contact, light),
            Self::Solid(solid) => solid.shade(ray, contact, light),
            Self::Textured(textured) => textured.shade(ray, contact, light),
            Self::Wireframe(wireframe) => wireframe.shade(ray, contact, light),
        }
    }
}

impl From<Angular> for Shader {
    #[inline]
    fn from(val: Angular) -> Self {
        Self::Angular(val)
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

impl From<Gradient> for Shader {
    #[inline]
    fn from(val: Gradient) -> Self {
        Self::Gradient(val)
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

impl From<Textured> for Shader {
    #[inline]
    fn from(val: Textured) -> Self {
        Self::Textured(val)
    }
}

impl From<Wireframe> for Shader {
    #[inline]
    fn from(val: Wireframe) -> Self {
        Self::Wireframe(val)
    }
}
