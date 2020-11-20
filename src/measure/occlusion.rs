//! Visibility calculation.

use crate::{
    geom::Ray,
    phys::Crossing,
    sim::render::{Attribute, Input},
};

/// Calculate the occlusion experienced over a distance along ray.
/// Zero indicates full occlusion.
/// Unity indicates full view.
#[inline]
#[must_use]
pub fn occlusion(input: &Input, mut ray: Ray, mut dist: f64) -> f64 {
    debug_assert!(dist > 0.0);

    let bump_dist = input.sett.bump_dist();
    let loop_limit = input.sett.loop_limit();
    let min_weight = input.sett.min_weight();

    let mut vis = 1.0;
    let mut num_loops = 0;
    while let Some(hit) = input.tree.scan(ray.clone(), bump_dist, dist) {
        // Loop limit check.
        if num_loops >= loop_limit {
            println!("[WARN] : Terminating shadower: loop limit reached.");
            return 0.0;
        }
        num_loops += 1;

        // Check if we've flown far enough.
        dist -= hit.dist();
        if dist < 0.0 {
            return vis;
        }

        // Check if it's still worth going.
        if vis < min_weight {
            return 0.0;
        }

        // Handle collision.
        match *hit.tag() {
            Attribute::Opaque(..) => {
                return vis / dist.mul_add(input.shader.fall_off(), 1.0);
            }
            Attribute::Mirror(.., abs_frac) => {
                ray.travel(dist);
                vis *= 1.0 - abs_frac;
                *ray.dir_mut() = Crossing::calc_ref_dir(ray.dir(), hit.side().norm());
                ray.travel(bump_dist);
            }
            Attribute::Transparent(.., abs_frac) => {
                ray.travel(dist + bump_dist);
                vis *= 1.0 - abs_frac;
            }
            Attribute::Refractive(.., abs_frac, [_inside, _outside]) => {
                ray.travel(dist + bump_dist);
                vis *= 1.0 - abs_frac;
            }
            Attribute::Luminous(.., bright_mult) => {
                return (vis * bright_mult) / dist.mul_add(input.shader.fall_off(), 1.0);
            }
        }
    }

    vis
}
