//! Engine sampling function.

use crate::{
    illumination::{light, shadow},
    input::Shader,
    parts::{Attributes, Scene, Tracer},
};
use arctk::{geom::Hit, math::Dir3, phys::Crossing};
use palette::{Gradient, LinSrgba};
use rand::rngs::ThreadRng;
use std::fmt::Display;

/// Sample the scene using the tracer.
#[inline]
#[must_use]
pub fn paint<T: Display + Ord>(
    mut rng: &mut ThreadRng,
    scene: &Scene<T>,
    shader: &Shader,
    mut trace: Tracer,
) -> LinSrgba {
    debug_assert!(trace.weight() > 0.0);
    debug_assert!(trace.weight() <= 1.0);

    // Convenience.
    let bump_dist = scene.sett.bump_dist();
    let sun_pos = shader.sky().sun_pos();

    // Tracked items.
    let mut col = LinSrgba::default();

    // Event loop.
    loop {
        if trace.weight() <= scene.sett.min_weight() {
            break;
        }

        if let Some(hit) = scene.tree.observe(trace.ray().clone(), bump_dist, 1_000.0) {
            if let Some(attr) = scene.attrs.map().get(hit.tag()) {
                match attr {
                    Attributes::Luminous { mult } => {
                        trace.travel(hit.dist());
                        let sun_dir = Dir3::new_normalize(trace.pos() - sun_pos);
                        col += colour(&mut rng, scene, shader, &trace, &hit, &sun_dir)
                            * (mult * trace.weight()) as f32;
                        break;
                    }
                    Attributes::Transparent { abs } => {
                        trace.travel(hit.dist());
                        let sun_dir = Dir3::new_normalize(trace.pos() - sun_pos);
                        col += colour(&mut rng, scene, shader, &trace, &hit, &sun_dir)
                            * (*abs * trace.weight()) as f32;
                        *trace.weight_mut() *= 1.0 - *abs;
                        trace.travel(bump_dist);
                    }
                    Attributes::Mirror { abs } => {
                        trace.travel(hit.dist());
                        let sun_dir = Dir3::new_normalize(trace.pos() - sun_pos);
                        col += colour(&mut rng, scene, shader, &trace, &hit, &sun_dir)
                            * (*abs * trace.weight()) as f32;
                        *trace.weight_mut() *= 1.0 - *abs;
                        trace.set_dir(Crossing::calc_ref_dir(trace.dir(), hit.side().norm()));
                        trace.travel(bump_dist);
                    }
                    Attributes::Refractive {
                        abs,
                        inside,
                        outside,
                    } => {
                        trace.travel(hit.dist());
                        let sun_dir = Dir3::new_normalize(trace.pos() - sun_pos);
                        col += colour(&mut rng, scene, shader, &trace, &hit, &sun_dir)
                            * (*abs * trace.weight()) as f32;

                        let (n_curr, n_next) = if hit.side().is_inside() {
                            (*inside, *outside)
                        } else {
                            (*outside, *inside)
                        };
                        let crossing =
                            Crossing::new(trace.ray().dir(), hit.side().norm(), n_curr, n_next);

                        // Transmission ray.
                        if let Some(trans_dir) = crossing.trans_dir() {
                            let mut trans = trace.clone();
                            *trans.weight_mut() *= crossing.trans_prob();
                            trans.set_dir(*trans_dir);
                            trans.travel(bump_dist);

                            col += paint(&mut rng, scene, shader, trans);
                        }

                        // Reflection ray.
                        *trace.weight_mut() *= crossing.ref_prob();
                        trace.set_dir(*crossing.ref_dir());
                        trace.travel(bump_dist);
                    }
                }
            } else {
                trace.travel(hit.dist());
                let sun_dir = Dir3::new_normalize(trace.pos() - sun_pos);
                col +=
                    colour(&mut rng, scene, shader, &trace, &hit, &sun_dir) * trace.weight() as f32;
                break;
            }
        } else {
            // col += sky_col(shader, trace.ray(), &scene.cols.map()["sky"]);
            break;
        }
    }

    col
}

/// Perform a colouring.
#[inline]
fn colour<T: Display + Ord>(
    rng: &mut ThreadRng,
    scene: &Scene<T>,
    shader: &Shader,
    trace: &Tracer,
    hit: &Hit<&T>,
    sun_dir: &Dir3,
) -> LinSrgba {
    let light = light(shader, trace.ray(), hit);
    let shadow = shadow(scene, shader, trace.ray(), hit, rng);

    let x = hit.side().norm().dot(sun_dir).abs();

    let base_col = scene.cols.map()[hit.tag()].get(x as f32);
    let grad = Gradient::new(vec![LinSrgba::default(), base_col]);

    grad.get((light * shadow) as f32)
}
