//! Live window rendering.

use crate::{
    input::Shader,
    output::Data,
    parts::{Camera, Scene, Tracer},
    run::engine::paint,
};
use arctk::{
    err::Error,
    ord::{BLUE, GREEN, RED},
    tools::{ProgressBar, SilentProgressBar},
};
use minifb::{Scale, Window, WindowOptions};
use palette::{LinSrgba, Pixel};
use rand::thread_rng;
use rayon::prelude::*;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

/// Render an image in a live window.
/// # Errors
/// if a mutex unwrapping failed or
/// an arc unwrapping failed or
/// if the progress bar can not be locked inside a running thread.
#[allow(clippy::expect_used)]
#[inline]
pub fn window_thread<T: Display + Ord + Sync>(
    update_size: u64,
    window_scale: Scale,
    scene: &Scene<T>,
    shader: &Shader,
    cam: &Camera,
) -> Result<Data, Error> {
    debug_assert!(update_size > 0);

    let num_pixels = cam.sensor().num_pixels();
    let mut main_bar = ProgressBar::new("Rendering", num_pixels as u64);

    let order = scene.sett.order().gen_list(num_pixels);

    let w = cam.sensor().res().0 as usize;
    let h = cam.sensor().res().1 as usize;

    let buffer: Vec<u32> = vec![0; w * h];
    let buffer = Arc::new(Mutex::new(buffer));
    let window_options = WindowOptions {
        borderless: true,
        title: true,
        resize: false,
        scale: window_scale,
        scale_mode: minifb::ScaleMode::Stretch,
        topmost: true,
        transparency: true,
    };
    let mut window = Window::new("Antler", w, h, window_options).unwrap_or_else(|e| {
        panic!("Could not create gui window: {}", e);
    });
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let data = Data::new([w, h]);
    let data = Arc::new(Mutex::new(data));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    while let Some((start, end)) = main_bar.block(update_size) {
        let pb = SilentProgressBar::new(end - start);
        let pb = Arc::new(Mutex::new(pb));

        while !pb.lock()?.is_done() {
            threads
                .par_iter()
                .map(|_id| {
                    render_range(
                        start,
                        &order,
                        &Arc::clone(&pb),
                        scene,
                        shader,
                        cam,
                        &Arc::clone(&data),
                        &Arc::clone(&buffer),
                    )
                })
                .collect::<()>();
        }

        window
            .update_with_buffer(&buffer.lock()?, w, h)
            .expect("Could not update window buffer.");
    }
    main_bar.finish_with_message("Render complete.");

    if let Ok(d) = Arc::try_unwrap(data) {
        return Ok(d.into_inner()?);
    }

    unreachable!("Failed to unwrap data.");
}

/// Render pixels using a single thread.
/// # Errors
/// if the progress bar can not be locked
#[allow(clippy::expect_used)]
#[inline]
fn render_range<T: Display + Ord>(
    offset: u64,
    order: &[u64],
    pb: &Arc<Mutex<SilentProgressBar>>,
    scene: &Scene<T>,
    shader: &Shader,
    cam: &Camera,
    data: &Arc<Mutex<Data>>,
    buffer: &Arc<Mutex<Vec<u32>>>,
) {
    let super_samples = cam.sensor().super_samples();
    let h_res = cam.sensor().res().0;
    let total_pixels = cam.sensor().num_pixels();
    let block_size = (scene.sett.block_size() / super_samples as u64).max(1);

    let weight = 1.0 / f64::from(super_samples);

    let mut rng = thread_rng();

    if let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let block = pb.block(block_size);
        std::mem::drop(pb);
        block
    } {
        for i in start..end {
            let p = i + offset;
            let p = order[p as usize];
            let pixel = [(p % h_res) as usize, (p / h_res) as usize];

            let mut total_col = LinSrgba::new(0.0, 0.0, 0.0, 0.0);
            for sub_sample in 0..super_samples {
                let ray = cam.gen_ray(pixel, sub_sample);

                let col = paint(&mut rng, scene, shader, cam, Tracer::new(ray));
                total_col += col * weight as f32;
            }

            data.lock().expect("Could not lock data.").img.pixels_mut()[pixel] += total_col;
            let raw_col: [u8; 4] = total_col.into_format().into_raw();
            buffer.lock().expect("Could not lock window buffer.")
                [(total_pixels - 1 - p) as usize] =
                components_to_u32(raw_col[RED], raw_col[GREEN], raw_col[BLUE]);
        }
    }
}

/// Create a 32 bit colour representation from 8 bit components.
#[inline]
#[must_use]
pub const fn components_to_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
