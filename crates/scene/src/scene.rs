use antler_colour::Rgb;
use antler_geometry::{Bounded, Bvh, Contact, Ray, Sample, Sampleable, Traceable, utils::hemisphere_direction};
use antler_id::ObjectId;
use antler_light::{Emissive, Light, LightSample};
use antler_material::Bsdf;
use antler_settings::OcclusionSettings;
use antler_shader::Appearance;
use antler_skybox::{Constant, Skybox};
use nalgebra::Unit;
use rand::Rng;

use crate::{object::Object, resources::Resources};

const MAX_VISIBILITY_HITS: usize = 16;
const VISIBILITY_EPSILON: f32 = 1.0e-3;

pub struct Scene {
    ambient: Rgb,
    skybox: Skybox,
    occlusion: Option<OcclusionSettings>,
    lights: Vec<Light>,
    objects: Vec<Object>,
    emissive_objects: Vec<ObjectId>,
    bvh: Option<Bvh<ObjectId>>,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ambient: Rgb::WHITE,
            skybox: Constant::new(Rgb::WHITE).into(),
            occlusion: None,
            lights: Vec::new(),
            objects: Vec::new(),
            emissive_objects: Vec::new(),
            bvh: None,
        }
    }

    #[inline]
    pub fn set_skybox(&mut self, skybox: Skybox) {
        self.skybox = skybox;
    }

    #[inline]
    pub const fn set_ambient(&mut self, ambient: Rgb) {
        self.ambient = ambient;
    }

    #[inline]
    pub const fn set_occlusion(&mut self, occlusion: Option<OcclusionSettings>) {
        self.occlusion = occlusion;
    }

    #[inline]
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    #[inline]
    pub fn add_object(&mut self, object: Object) {
        let object_id = ObjectId::new(self.objects.len());

        if object.emissive.is_some() {
            self.emissive_objects.push(object_id);
        }

        self.objects.push(object);
        self.bvh = None;
    }

    #[must_use]
    pub fn get_skybox(&self) -> &Skybox {
        &self.skybox
    }

    #[must_use]
    #[inline]
    pub fn get_light(&self, light_id: ObjectId) -> &Light {
        &self.lights[light_id.index()]
    }

    #[must_use]
    #[inline]
    pub fn get_object(&self, object_id: ObjectId) -> &Object {
        &self.objects[object_id.index()]
    }

    #[must_use]
    #[inline]
    pub const fn bvh(&self) -> &Bvh<ObjectId> {
        self.bvh.as_ref().expect("Scene BVH is not built")
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

    #[must_use]
    #[inline]
    pub fn hit(&self, resources: &Resources, world_ray: &Ray, max_distance: f32) -> Option<ObjectId> {
        let bvh = self.bvh();
        let mut limit = max_distance;
        let mut contact = None;

        bvh.any_with_limit(world_ray, &mut limit, |object_id, limit| {
            if self.hit_object(resources, object_id, world_ray, *limit) {
                contact = Some(object_id);
                true
            } else {
                false
            }
        });

        contact
    }

    #[must_use]
    #[inline]
    pub fn distance(&self, resources: &Resources, world_ray: &Ray, max_distance: f32) -> Option<(ObjectId, f32)> {
        let bvh = self.bvh();
        let mut nearest = None;
        let mut best_distance = max_distance;

        bvh.nearest_with_max(world_ray, &mut best_distance, |object_id, best_distance| {
            if let Some(distance) = self.distance_object(resources, object_id, world_ray, *best_distance) {
                *best_distance = distance;
                nearest = Some((object_id, distance));
            }

            true
        });

        nearest
    }

    #[must_use]
    #[inline]
    pub fn visibility(&self, resources: &Resources, world_ray: &Ray, max_distance: f32) -> Rgb {
        let mut visibility = Rgb::WHITE;
        let mut ray = *world_ray;
        let mut remaining_distance = max_distance;

        for _ in 0..MAX_VISIBILITY_HITS {
            let Some((object_id, contact)) = self.intersection(resources, &ray, remaining_distance) else {
                break;
            };

            let object = self.get_object(object_id);
            let material = resources.get_material(object.material_id);
            let shader = resources.get_shader(object.shader_id);

            let material_visibility = material.visibility();

            if material_visibility.luminance() <= VISIBILITY_EPSILON {
                return Rgb::BLACK;
            }

            visibility *= material_visibility * shader.colour(&ray.direction, &contact);

            if visibility.luminance() <= VISIBILITY_EPSILON {
                return Rgb::BLACK;
            }

            remaining_distance -= contact.distance;

            if remaining_distance <= 0.0 {
                break;
            }

            ray = Ray::from_offset(contact.position, contact.normal, ray.direction);
        }

        visibility
    }

    #[must_use]
    #[inline]
    pub fn intersection(
        &self,
        resources: &Resources,
        world_ray: &Ray,
        max_distance: f32,
    ) -> Option<(ObjectId, Contact)> {
        let bvh = self.bvh();
        let mut nearest = None;
        let mut best_distance = max_distance;

        bvh.nearest_with_max(world_ray, &mut best_distance, |object_id, best_distance| {
            if let Some(contact) = self.intersection_object(resources, object_id, world_ray, *best_distance) {
                *best_distance = contact.distance;
                nearest = Some((object_id, contact));
            }

            true
        });

        nearest
    }

    #[must_use]
    #[inline]
    pub fn hit_object(&self, resources: &Resources, object_id: ObjectId, world_ray: &Ray, max_distance: f32) -> bool {
        self.distance_object(resources, object_id, world_ray, max_distance)
            .is_some()
    }

    #[must_use]
    #[inline]
    pub fn distance_object(
        &self,
        resources: &Resources,
        object_id: ObjectId,
        world_ray: &Ray,
        max_distance: f32,
    ) -> Option<f32> {
        let object = self.get_object(object_id);
        let object_ray = world_ray.transform(&object.inv_transform);

        let scale = object.transform.scaling();
        let object_max_distance = max_distance / scale;

        let geometry = resources.get_geometry(object.geometry_id);
        geometry
            .distance(&object_ray, object_max_distance)
            .map(|object_distance| object_distance * scale)
            .filter(|world_distance| *world_distance < max_distance)
    }

    #[must_use]
    #[inline]
    pub fn intersection_object(
        &self,
        resources: &Resources,
        object_id: ObjectId,
        world_ray: &Ray,
        max_distance: f32,
    ) -> Option<Contact> {
        let object = self.get_object(object_id);
        let object_ray = world_ray.transform(&object.inv_transform);

        let scale = object.transform.scaling();
        let object_max_distance = max_distance / scale;

        let geometry = resources.get_geometry(object.geometry_id);
        geometry
            .intersection(&object_ray, object_max_distance)
            .map(|contact| contact.transform(&object.transform, world_ray.origin))
            .filter(|contact| contact.distance < max_distance)
    }

    #[must_use]
    #[inline]
    pub fn ambient_shade(&self, shader: &impl Appearance, world_ray: &Ray, contact: &Contact) -> Rgb {
        let sample = LightSample {
            direction: contact.normal,
            radiance: self.ambient,
            distance: f32::INFINITY,
        };

        shader.shade(world_ray, contact, &sample)
    }

    #[must_use]
    #[inline]
    pub fn direct_light<R: Rng>(
        &self,
        rng: &mut R,
        resources: &Resources,
        world_ray: &Ray,
        object_id: ObjectId,
        contact: &mut Contact,
    ) -> Rgb {
        let object = self.get_object(object_id);
        let shader = resources.get_shader(object.shader_id);

        let mut total = Rgb::BLACK;

        // Analytic lights
        for light in &self.lights {
            let mut light_total = Rgb::BLACK;
            let mut sample_count = 0usize;

            light.for_each_sample(rng, contact, |sample| {
                sample_count += 1;

                let n_dot_l = contact.normal.dot(&sample.direction);
                if n_dot_l <= 0.0 {
                    return;
                }

                let shadow_ray = Ray::from_offset(contact.position, contact.normal, sample.direction);
                let shadow_distance = sample.distance - (shadow_ray.origin - contact.position).norm();

                if shadow_distance > 0.0 {
                    let visibility = self.visibility(resources, &shadow_ray, shadow_distance);

                    if visibility.luminance() > VISIBILITY_EPSILON {
                        light_total += shader.shade(world_ray, contact, &sample) * visibility;
                    }
                }
            });

            if sample_count > 0 {
                total += light_total / sample_count as f32;
            }
        }

        // Emissive objects
        for &emissive_object_id in &self.emissive_objects {
            if emissive_object_id == object_id {
                continue;
            }

            let emissive_object = self.get_object(emissive_object_id);

            let Some(emissive) = &emissive_object.emissive else {
                continue;
            };

            let geometry = resources.get_geometry(emissive_object.geometry_id);
            let radiance = emissive.colour * emissive.intensity;

            let samples = emissive.samples.max(1);
            let mut light_total = Rgb::BLACK;

            for _ in 0..samples {
                let surface_sample = geometry.sample(rng).transform(&emissive_object.transform);

                let Some(light_sample) = area_light_sample(contact, &surface_sample, radiance) else {
                    continue;
                };

                if contact.normal.dot(&light_sample.direction) <= 0.0 {
                    continue;
                }

                let shadow_ray = Ray::from_offset(contact.position, contact.normal, light_sample.direction);
                let shadow_distance = light_sample.distance - (shadow_ray.origin - contact.position).norm() - 1.0e-4;

                if shadow_distance > 0.0 {
                    let visibility = self.visibility(resources, &shadow_ray, shadow_distance);

                    if visibility.luminance() > VISIBILITY_EPSILON {
                        light_total += shader.shade(world_ray, contact, &light_sample) * visibility;
                    }
                }
            }

            if samples > 0 {
                total += light_total / samples as f32;
            }
        }

        total
    }

    #[must_use]
    #[inline]
    pub fn occlusion<R: Rng>(&self, rng: &mut R, resources: &Resources, contact: &mut Contact) -> f32 {
        let Some(ref ao) = self.occlusion else {
            return 1.0;
        };

        if ao.samples == 0 || ao.distance <= 0.0 || ao.strength <= 0.0 {
            return 1.0;
        }

        let mut occlusion = 0.0;

        for _ in 0..ao.samples {
            let direction = hemisphere_direction(rng, contact.normal, contact.tangent(), contact.bi_tangent());
            let ray = Ray::from_offset(contact.position, contact.normal, direction);

            if let Some((_object_id, distance)) = self.distance(resources, &ray, ao.distance) {
                let proximity = 1.0 - (distance / ao.distance).clamp(0.0, 1.0);
                occlusion += proximity.powf(ao.falloff);
            }
        }

        let occlusion = occlusion / ao.samples as f32;

        ao.strength.mul_add(-occlusion, 1.0).clamp(0.0, 1.0)
    }
}

#[must_use]
#[inline]
fn area_light_sample(contact: &Contact, sample: &Sample, emission: Rgb) -> Option<LightSample> {
    let to_light = sample.position - contact.position;
    let distance_squared = to_light.norm_squared();

    if distance_squared <= 1.0e-8 {
        return None;
    }

    let distance = distance_squared.sqrt();
    let direction = Unit::new_normalize(to_light);

    let cos_light = sample.normal.dot(&-direction).max(0.0);

    if cos_light <= 0.0 {
        return None;
    }

    let pdf_solid_angle = sample.pdf_area * distance_squared / cos_light;

    Some(LightSample {
        direction,
        distance,
        radiance: emission / pdf_solid_angle,
    })
}
