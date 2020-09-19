//! Live window rendering.

use crate::{
    input::Shader,
    output::Data,
    parts::{Camera, Scene},
    // run::engine::paint,
};
use arctk::{err::Error, tools::ProgressBar};
// use palette::LinSrgba;
// use rand::thread_rng;
// use rayon::prelude::*;
use minifb::{Key, Window, WindowOptions};
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

/// Render an image in a live window.
/// # Errors
/// if a mutex unwrapping failed or
/// an arc unwrapping failed or
/// if the progress bar can not be locked inside a running thread.
#[inline]
pub fn window_thread<T: Display + Ord + Sync>(
    _scene: &Scene<T>,
    _shader: &Shader,
    cam: &Camera,
) -> Result<Data, Error> {
    let num_pixels = cam.sensor().num_pixels();
    let pb = ProgressBar::new("Rendering", num_pixels as u64);
    let _pb = Arc::new(Mutex::new(pb));

    let w = cam.sensor().res().0 as usize;
    let h = cam.sensor().res().1 as usize;

    let mut buffer: Vec<u32> = vec![0; w * h];
    let window_options = WindowOptions {
        borderless: true,
        title: true,
        resize: false,
        scale: minifb::Scale::X1,
        scale_mode: minifb::ScaleMode::Stretch,
        topmost: true,
        transparency: true,
    };
    let mut window = Window::new("Antler", w, h, window_options).unwrap_or_else(|e| {
        panic!("Could not create gui window: {}", e);
    });

    // let threads: Vec<_> = (0..num_cpus::get()).collect();
    // let mut out: Vec<_> = threads
    //     .par_iter()
    //     .map(|_id| run_thread(&Arc::clone(&pb), scene, shader, cam))
    //     .collect();
    // pb.lock()?.finish_with_message("Render complete.");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, w, h).unwrap();
    }

    // let mut data = out.pop().ok_or("No data received.")??;
    // while let Some(o) = out.pop() {
    //     data += &o?;
    // }

    let data = Data::new([w, h]);
    Ok(data)
}
