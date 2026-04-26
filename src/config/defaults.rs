use crate::{
    colour::{Rgb, Rgba},
    config::Vec3,
};

pub fn one_f32() -> f32 {
    1.0
}

pub fn one_usize() -> usize {
    1
}

pub fn unit_square() -> [f32; 2] {
    [1.0, 1.0]
}

pub fn z_axis() -> Vec3 {
    Vec3([0.0, 0.0, 1.0])
}

pub fn white() -> Rgb {
    Rgb::WHITE
}

pub fn transparent() -> Rgba {
    Rgba::TRANSPARENT
}
