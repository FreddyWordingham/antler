//! Fast-scheme rendering function.

use crate::{input::Shader, output::Data, parts::Tracer, run::Scene};
use arctk::{err::Error, tools::ProgressBar};
use palette::LinSrgba;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

/// Render an image as fast as possible.
/// # Errors
/// if a mutex unwrapping failed or
/// an arc unwrapping failed.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread<T: Ord>(scene: &Scene<T>, shader: &Shader) -> Result<Data, Error> {
    let num_pixels = shader.cam().sensor().num_pixels();
    let pb = ProgressBar::new("Rendering", num_pixels as u64);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| run_thread(&Arc::clone(&pb), scene, shader))
        .collect();
    pb.lock()?.finish_with_message("Render complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Render an image using a single thread.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
pub fn single_thread<T: Ord>(scene: &Scene<T>, shader: &Shader) -> Data {
    let num_pixels = shader.cam().sensor().num_pixels();
    let pb = ProgressBar::new("Rendering", num_pixels as u64);
    let pb = Arc::new(Mutex::new(pb));

    run_thread(&pb, scene, shader)
}

/// Render pixels using a single thread.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn run_thread<T: Ord>(pb: &Arc<Mutex<ProgressBar>>, scene: &Scene<T>, shader: &Shader) -> Data {
    let w = shader.cam().sensor().res().0 as usize;
    let h = shader.cam().sensor().res().1 as usize;

    let super_samples = shader.cam().sensor().super_samples();
    let dof_samples = shader.cam().focus().dof_samples();
    let h_res = shader.cam().sensor().res().0;

    let weight = 1.0 / f64::from(super_samples * dof_samples);

    let mut rng = thread_rng();

    let mut data = Data::new([w, h]);
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress pb.");
        let b = pb.block(scene.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for p in start..end {
            let pixel = [(p % h_res) as usize, (p / h_res) as usize];

            let mut total_col = LinSrgba::default();
            for sub_sample in 0..super_samples {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for depth_sample in 0..dof_samples {
                    let ray = shader
                        .cam()
                        .gen_ray(pixel, offset, sub_sample, depth_sample);

                    let sample = super::paint(&mut rng, scene, shader, Tracer::new(ray));
                    total_col += sample.col * weight as f32;
                }
            }

            data.img.pixels_mut()[pixel] = total_col;
        }
    }

    data
}
