use antler_geometry::Contact;
use rand::Rng;

use crate::light_sample::LightSample;

pub trait Emissive {
    fn for_each_sample<R: Rng, F: FnMut(LightSample)>(&self, rng: &mut R, contact: &Contact, f: F);
}
