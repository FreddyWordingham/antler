use crate::{
    colour::Rgb,
    lighting::LightSample,
    shader::Shader,
    tracing::{WorldHit, WorldRay},
};

pub struct Checkerboard {
    pub size: f32,
    pub colour_a: Rgb,
    pub colour_b: Rgb,
}

impl Checkerboard {
    pub fn new(size: f32, colour_a: Rgb, colour_b: Rgb) -> Self {
        Self {
            size,
            colour_a,
            colour_b,
        }
    }
}

impl Shader for Checkerboard {
    fn emitted(&self, _hit: &WorldHit) -> Rgb {
        Rgb::BLACK
    }

    fn shade(&self, _ray: &WorldRay, hit: &WorldHit, light: &LightSample) -> Rgb {
        let colour = if ((hit.position.x / self.size).floor()
            + (hit.position.y / self.size).floor()
            + (hit.position.z / self.size).floor()) as i32
            % 2
            == 0
        {
            self.colour_a
        } else {
            self.colour_b
        };

        let n_dot_l = hit.normal.dot(&light.direction).max(0.0);
        colour * light.radiance * n_dot_l
    }
}
