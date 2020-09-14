//! Visibility calculation.

use crate::parts::{Attributes, Scene, Tracer};
use arctk::phys::Crossing;
use std::fmt::Display;

/// Calculate the visibility of a given tracer.
/// Tracer is traced to a maximum of the visibility distance.
#[inline]
#[must_use]
pub fn visibility<T: Display + Ord>(scene: &Scene<T>, mut trace: Tracer, mut vis: f64) -> f64 {
    debug_assert!(vis > 0.0);
    debug_assert!(vis <= 1.0);

    let bump_dist = scene.sett.bump_dist();
    let visibility_dist = scene.sett.visibility_dist();

    while let Some(hit) = scene.tree.observe(
        trace.ray().clone(),
        bump_dist,
        visibility_dist - trace.dist_travelled(),
    ) {
        let tag = hit.tag();
        if let Some(attr) = scene.attrs.map().get(tag) {
            match attr {
                Attributes::Luminous => {
                    return 1.0;
                }
                Attributes::Transparent { abs } | Attributes::Refractive { abs, .. } => {
                    vis *= 1.0 - *abs;
                    trace.travel(hit.dist() + bump_dist);
                }
                Attributes::Mirror { abs } => {
                    trace.travel(hit.dist());
                    vis *= 1.0 - *abs;
                    trace.set_dir(Crossing::calc_ref_dir(trace.dir(), hit.side().norm()));
                    trace.travel(bump_dist);
                }
            }
        } else {
            return 0.0;
        }

        if trace.dist_travelled() >= visibility_dist {
            break;
        }
    }

    vis
}
