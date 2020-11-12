//! Rendering thread control.

use crate::{
    input::Shader,
    output::Data,
    parts::{Camera, Scene, Tracer},
    run::engine::paint,
};
use arctk::{err::Error, math::Vec3, tools::ProgressBar};
use palette::LinSrgba;
use rand::thread_rng;
use rayon::prelude::*;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
    time::Instant,
};

/// Render an image as fast as possible.
/// # Errors
/// if a mutex unwrapping failed or
/// an arc unwrapping failed or
/// if the progress bar can not be locked inside a running thread.
#[inline]
pub fn multi_thread<T: Display + Ord + Sync>(
    scene: &Scene<T>,
    shader: &Shader,
    cam: &Camera,
) -> Result<Data, Error> {
    let num_pixels = cam.sensor().num_pixels();
    let pb = ProgressBar::new("Rendering", num_pixels as u64);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| run_thread(&Arc::clone(&pb), scene, shader, cam))
        .collect();
    pb.lock()?.finish_with_message("Render complete.");

    let mut data = out.pop().ok_or("No data received.")??;
    while let Some(o) = out.pop() {
        data += &o?;
    }

    Ok(data)
}

/// Render an image using a single thread.
/// # Errors
/// if the progress bar can not be locked
#[inline]
pub fn single_thread<T: Display + Ord>(
    scene: &Scene<T>,
    shader: &Shader,
    cam: &Camera,
) -> Result<Data, Error> {
    let num_pixels = cam.sensor().num_pixels();
    let pb = ProgressBar::new("Rendering", num_pixels as u64);
    let pb = Arc::new(Mutex::new(pb));

    run_thread(&pb, scene, shader, cam)
}

/// Render pixels using a single thread.
/// # Errors
/// if the progress bar can not be locked
#[inline]
fn run_thread<T: Display + Ord>(
    pb: &Arc<Mutex<ProgressBar>>,
    scene: &Scene<T>,
    shader: &Shader,
    cam: &Camera,
) -> Result<Data, Error> {
    let w = cam.sensor().res().0 as usize;
    let h = cam.sensor().res().1 as usize;

    let super_samples = cam.sensor().super_samples();
    let h_res = cam.sensor().res().0;
    let block_size = (scene.sett.block_size() / super_samples as u64).max(1);

    let weight = 1.0 / f64::from(super_samples);

    let mut rng = thread_rng();

    let mut data = Data::new([w, h]);
    while let Some((start, end)) = {
        let mut pb = pb.lock()?;
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for p in start..end {
            let pixel = [(p % h_res) as usize, (p / h_res) as usize];

            let mut total_col = LinSrgba::new(0.0, 0.0, 0.0, 0.0);
            let mut total_dist = 0.0;
            let mut total_dir = Vec3::default();
            let start_time = Instant::now();

            for sub_sample in 0..super_samples {
                let ray = cam.gen_ray(pixel, sub_sample);

                let (col, dist, dir) = paint(&mut rng, scene, shader, cam, Tracer::new(ray));
                total_col += col * weight as f32;
                total_dist += dist * weight;
                total_dir += dir * weight;
            }
            let calc_time = start_time.elapsed().as_micros();

            data.end_dir[pixel] += total_dir;
            data.dist[pixel] += total_dist;
            data.time[pixel] += calc_time as f64;
            data.img.pixels_mut()[pixel] += total_col;
        }
    }

    Ok(data)
}
