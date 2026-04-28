use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;

pub trait Appearance {
    fn emitted(&self, contact: &Contact) -> Rgb;

    fn albedo(&self, contact: &Contact) -> Rgb;

    fn shade(&self, ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb;
}
