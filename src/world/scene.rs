use std::f32::INFINITY;

use rand::Rng;

use crate::{
    acceleration::Bvh,
    colour::Rgb,
    geometry::{Bounded, Traceable},
    id::ObjectId,
    lighting::{Light, LightEnum},
    shader::Shader,
    tracing::{WorldHit, WorldRay},
    utils::sampling,
    world::{Object, World},
};

pub struct Scene {
    ambient: Rgb,
    ambient_occlusion: Option<AmbientOcclusion>,
    lights: Vec<LightEnum>,
    objects: Vec<Object>,
    bvh: Option<Bvh<ObjectId>>,
}

#[derive(Debug, Clone, Copy)]
pub struct AmbientOcclusion {
    pub samples: usize,
    pub distance: f32,
    pub strength: f32,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new(Rgb::WHITE, None)
    }
}

impl Scene {
    pub fn new(ambient: Rgb, ambient_occlusion: Option<AmbientOcclusion>) -> Self {
        Self {
            ambient,
            ambient_occlusion,
            lights: Vec::new(),
            objects: Vec::new(),
            bvh: None,
        }
    }

    #[inline]
    pub fn add_light<L>(&mut self, light: L)
    where
        L: Into<LightEnum>,
    {
        self.lights.push(light.into());
    }

    #[inline]
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
        self.bvh = None;
    }

    #[inline]
    pub fn get_object(&self, id: ObjectId) -> &Object {
        &self.objects[id.index()]
    }

    pub fn trace(&self, world: &World, world_ray: &WorldRay) -> Option<WorldHit> {
        let bvh = self.bvh.as_ref().expect("Must build BVH before tracing.");

        let mut nearest = None;
        let mut best_distance = INFINITY;

        bvh.trace_nearest_with_max(world_ray, &mut best_distance, |object_id, best_distance| {
            let Some(world_hit) = self.trace_object(world, object_id, world_ray) else {
                return true;
            };

            if world_hit.distance < *best_distance {
                *best_distance = world_hit.distance;
                nearest = Some(world_hit);
            }

            true
        });

        nearest
    }

    pub fn occluded(&self, world: &World, world_ray: &WorldRay, max_distance: f32) -> bool {
        let bvh = self.bvh.as_ref().expect("Must build BVH before tracing.");

        let mut max_distance = max_distance;

        bvh.trace_any_with_limit(world_ray, &mut max_distance, |object_id, max_distance| {
            let Some(world_hit) = self.trace_object(world, object_id, world_ray) else {
                return false;
            };

            if world_hit.distance < *max_distance {
                *max_distance = world_hit.distance;
                true
            } else {
                false
            }
        })
    }

    pub fn ambient_light(&self, world: &World, hit: &WorldHit, rng: &mut impl Rng) -> Rgb {
        let object = self.get_object(hit.object_id);
        let shader = world.get_shader(object.shader_id);

        shader.albedo(hit) * self.ambient * self.ambient_occlusion_factor(world, hit, rng)
    }

    fn ambient_occlusion_factor(&self, world: &World, hit: &WorldHit, rng: &mut impl Rng) -> f32 {
        let Some(ao) = self.ambient_occlusion else {
            return 1.0;
        };

        if ao.samples == 0 || ao.distance <= 0.0 || ao.strength <= 0.0 {
            return 1.0;
        }

        let occluded = (0..ao.samples)
            .filter(|_| {
                let direction = sampling::hemisphere_direction(hit.normal, rng);
                let ray = WorldRay::from_offset(hit.position, hit.normal, direction);
                self.occluded(world, &ray, ao.distance)
            })
            .count();

        let occlusion = occluded as f32 / ao.samples as f32;
        (1.0 - ao.strength * occlusion).clamp(0.0, 1.0)
    }

    pub fn direct_light(&self, world: &World, world_ray: &WorldRay, hit: &WorldHit, rng: &mut impl Rng) -> Rgb {
        let object = self.get_object(hit.object_id);
        let shader = world.get_shader(object.shader_id);

        let mut total = Rgb::BLACK;

        for light in &self.lights {
            let mut light_total = Rgb::BLACK;
            let mut sample_count = 0usize;

            light.for_each_sample(hit, rng, |sample| {
                sample_count += 1;

                let n_dot_l = hit.normal.dot(&sample.direction);
                if n_dot_l <= 0.0 {
                    return;
                }

                let shadow_ray = WorldRay::from_offset(hit.position, hit.normal, sample.direction);

                if !self.occluded(world, &shadow_ray, sample.distance) {
                    light_total += shader.shade(hit, world_ray, &sample);
                }
            });

            if sample_count > 0 {
                total += light_total / sample_count as f32;
            }
        }

        total
    }

    pub fn build(&mut self, world: &World) {
        let items = self
            .objects
            .iter()
            .enumerate()
            .map(|(index, object)| {
                let bounds = world
                    .get_geometry(object.geometry_id)
                    .bounds()
                    .transform(&object.transform);

                (bounds, ObjectId::new(index))
            })
            .collect();

        self.bvh = Some(Bvh::new(items));
    }

    fn trace_object(&self, world: &World, object_id: ObjectId, world_ray: &WorldRay) -> Option<WorldHit> {
        let object = self.get_object(object_id);
        let object_ray = world_ray.to_object_space(&object.inv_transform);

        let geometry = world.get_geometry(object.geometry_id);
        let object_hit = geometry.trace(&object_ray)?;

        Some(object_hit.to_world_space(&object.transform, world_ray.origin, object_id))
    }
}
