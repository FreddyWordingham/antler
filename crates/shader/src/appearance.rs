use antler_colour::Rgb;
use antler_geometry::{Contact, Ray};
use antler_light::LightSample;
use nalgebra::{Unit, Vector3};

pub trait Appearance {
    fn colour(&self, direction: &Unit<Vector3<f32>>, contact: &Contact) -> Rgb;

    fn emitted(&self, contact: &Contact) -> Rgb;

    fn shade(&self, ray: &Ray, contact: &Contact, light: &LightSample) -> Rgb;
}
