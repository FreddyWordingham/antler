use antler_colour::Rgb;
use antler_geometry::{Bounded, Bvh};
use antler_id::ObjectId;
use antler_light::Light;
use antler_settings::OcclusionSettings;

use crate::{object::Object, resources::Resources};

pub struct Scene {
    ambient: Rgb,
    occlusion: Option<OcclusionSettings>,
    lights: Vec<Light>,
    objects: Vec<Object>,
    bvh: Option<Bvh<ObjectId>>,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            ambient: Rgb::WHITE,
            occlusion: None,
            lights: Vec::new(),
            objects: Vec::new(),
            bvh: None,
        }
    }

    pub const fn set_ambient(&mut self, ambient: Rgb) {
        self.ambient = ambient;
    }

    pub const fn set_occlusion(&mut self, occlusion: Option<OcclusionSettings>) {
        self.occlusion = occlusion;
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
        self.bvh = None;
    }

    pub fn build(&mut self, resources: &Resources) {
        let items = self
            .objects
            .iter()
            .enumerate()
            .map(|(index, object)| {
                let bounds = resources
                    .get_geometry(object.geometry_id)
                    .bounds()
                    .transform(&object.transform);

                (bounds, ObjectId::new(index))
            })
            .collect();

        self.bvh = Some(Bvh::new(items));
    }
}
