use antler_colour::Rgb;
use antler_geometry::Bvh;
use antler_id::ObjectId;
use antler_light::Light;
use antler_settings::OcclusionSettings;

use crate::object::Object;

pub struct Scene {
    ambient: Rgb,
    occlusion: Option<OcclusionSettings>,
    lights: Vec<Light>,
    objects: Vec<Object>,
    bvh: Option<Bvh<ObjectId>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            ambient: Rgb::WHITE,
            occlusion: None,
            lights: Vec::new(),
            objects: Vec::new(),
            bvh: None,
        }
    }

    pub fn set_ambient(&mut self, ambient: Rgb) {
        self.ambient = ambient;
    }

    pub fn set_occlusion(&mut self, occlusion: Option<OcclusionSettings>) {
        self.occlusion = occlusion;
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
        self.bvh = None;
    }
}
