//! Rendering thread control.

use crate::{
    input::Shader,
    output::Data,
    parts::{Scene, Tracer},
    run::engine::paint,
};
use arctk::{err::Error, tools::ProgressBar};
use palette::LinSrgba;
use rand::thread_rng;
use rayon::prelude::*;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

/// Render an image as fast as possible.
/// # Errors
/// if a mutex unwrapping failed or
/// an arc unwrapping failed.
#[inline]
pub fn multi_thread<T: Display + Ord + Sync>(
    scene: &Scene<T>,
    shader: &Shader,
) -> Result<Data, Error> {
    let num_pixels = shader.cam().sensor().num_pixels();
    let pb = ProgressBar::new("Rendering", num_pixels as u64);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| run_thread(&Arc::clone(&pb), scene, shader))
        .collect();
    pb.lock()?.finish_with_message("Render complete.");

    let mut data = out.pop().ok_or("No data received.")??;
    while let Some(o) = out.pop() {
        data += &o?;
    }

    Ok(data)
}

/// Render an image using a single thread.
#[inline]
pub fn single_thread<T: Display + Ord>(scene: &Scene<T>, shader: &Shader) -> Result<Data, Error> {
    let num_pixels = shader.cam().sensor().num_pixels();
    let pb = ProgressBar::new("Rendering", num_pixels as u64);
    let pb = Arc::new(Mutex::new(pb));

    run_thread(&pb, scene, shader)
}

/// Render pixels using a single thread.
#[inline]
fn run_thread<T: Display + Ord>(
    pb: &Arc<Mutex<ProgressBar>>,
    scene: &Scene<T>,
    shader: &Shader,
) -> Result<Data, Error> {
    let w = shader.cam().sensor().res().0 as usize;
    let h = shader.cam().sensor().res().1 as usize;

    let super_samples = shader.cam().sensor().super_samples();
    let h_res = shader.cam().sensor().res().0;

    let weight = 1.0 / super_samples as f64;

    let mut rng = thread_rng();

    let mut data = Data::new([w, h]);
    while let Some((start, end)) = {
        let mut pb = pb.lock()?;
        let b = pb.block(scene.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for p in start..end {
            let pixel = [(p % h_res) as usize, (p / h_res) as usize];

            let mut total_col = LinSrgba::default();
            for sub_sample in 0..super_samples {
                let ray = shader.cam().gen_ray(pixel, sub_sample);

                let sample = paint(&mut rng, scene, shader, Tracer::new(ray));
                total_col += sample.col * weight as f32;
            }

            data.img.pixels_mut()[pixel] = total_col;
        }
    }

    Ok(data)
}
