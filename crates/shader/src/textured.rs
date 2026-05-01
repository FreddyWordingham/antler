use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_image::RgbImage;
use antler_light::LightSample;

use crate::Appearance;

pub struct Textured {
    image: RgbImage,
}

impl Textured {
    #[must_use]
    pub const fn new(image: RgbImage) -> Self {
        Self { image }
    }
}

impl Appearance for Textured {
    #[inline]
    fn emitted(&self, _contact: &Contact) -> Rgb {
        Rgb::BLACK
    }

    #[inline]
    fn shade(&self, _ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb {
        let colour = self.image.sample_nearest(contact.uv);
        let n_dot_l = contact.normal.dot(&light.direction).max(0.0);

        colour * light.radiance * n_dot_l
    }
}
