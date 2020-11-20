//! Simulation running functions.

use crate::{
    err::Error,
    ord::X,
    sim::render::{Engine, Input, Output, Tracer},
    tools::ProgressBar,
};
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a multi-threaded MCRT simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread<'a>(engine: Engine, input: &'a Input) -> Result<Output<'a>, Error> {
    let pb = ProgressBar::new("Multi-threaded", input.cam.num_samples());
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| thread(engine, input, &Arc::clone(&pb)))
        .collect();
    pb.lock()?.finish_with_message("Simulation complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Run a MCRT simulation using a single thread.
/// # Errors
/// if the progress bar can not be locked.
#[inline]
pub fn single_thread<'a>(engine: Engine, input: &'a Input) -> Result<Output<'a>, Error> {
    let pb = ProgressBar::new("Single-threaded", input.cam.num_samples());
    let pb = Arc::new(Mutex::new(pb));

    Ok(thread(engine, input, &pb))
}

/// Thread control function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn thread<'a>(engine: Engine, input: &'a Input, pb: &Arc<Mutex<ProgressBar>>) -> Output<'a> {
    let res = *input.cam.res();
    let mut data = Output::new(res, input.shader.data_grad());

    let mut rng = thread_rng();

    let super_samples = input.cam.num_super_samples();
    let ss_power = input.cam.ss_power();
    let init_weight = 1.0 / super_samples as f64;

    let block_size = input.sett.block_size();
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let p = n / super_samples;
            let s = n - (p * super_samples);

            let pixel = [p % res[X], p / res[X]];
            let ss = [s % ss_power, s / ss_power];

            let tracer = Tracer::new(input.cam.emit(pixel, ss), init_weight);
            engine(input, &mut rng, tracer, &mut data, pixel);
        }
    }

    data
}
