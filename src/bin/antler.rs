//! Antler rendering engine binary.
//! Produce image data from a given setup and camera.

use antler::{multi_thread, Input, ParametersBuilder};
use arctk::{
    args,
    file::{Build, Load, Save},
    geom::Tree,
    ord::Link,
    util::{
        banner::{section, title},
        dir,
    },
};
use std::{env::current_dir, path::PathBuf};

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);
    title(term_width, "Antler");

    section(term_width, "Initialisation");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    let cwd = current_dir().expect("Failed to determine current working directory.");
    // let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.clone()), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");

    section(term_width, "Input");
    let builder = ParametersBuilder::load(&in_dir.join(params_path))
        .expect("Failed to load parameters file.");

    section(term_width, "Building");
    let setup = builder
        .build(&in_dir)
        .expect("Failed to construct builder structure.");

    section(term_width, "Linking");
    let grads = setup.grads;
    let attrs = setup.attrs.link(&grads).expect("Gradient link failure.");
    let surfs = setup.surfs.link(&attrs).expect("Surface link failure.");
    let cam = setup.cam;
    let tree = Tree::new(&setup.tree, &surfs);
    let sett = setup.sett;
    let shader = setup.shader.link(&grads).expect("Gradient link Failure.");
    let engine = setup.engine;
    let input = Input::new(&grads, &attrs, &cam, &tree, &sett, &shader);

    section(term_width, "Simulation");
    // let output = single_thread(engine, &input).expect("Failed to run simulation");
    let output = multi_thread(engine, &input).expect("Failed to run simulation");
    output.save(&out_dir).expect("Failed to save output data.");

    section(term_width, "Finished");
}
