//! Rendering engine binary.

use arctk::{
    args,
    err::Error,
    geom::{MeshBuilder, TreeBuilder},
    img::GradientBuilder,
    ord::Set,
    util::{banner, dir, exec},
};
use arctk_attr::input;
use palette::{Gradient, LinSrgba};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Key type.
type Key = String;

/// Input parameters.
#[input]
struct Parameters {
    /// Adaptive mesh settings.
    tree: TreeBuilder,
    // /// Render runtime settings.
    // sett: render::Settings,
    /// Surfaces map.
    surfs: Set<Key, MeshBuilder>,
    // /// Attributes map.
    // attrs: Set<render::Attributes>,
    /// Colour map.
    cols: Set<Key, GradientBuilder>,
    // /// Shader.
    // shader: render::ShaderBuilder,
}

fn main() {
    banner::title("RENDER").expect("Failed to print title.");
    let (params_path, in_dir, out_dir) = init();
}

/// Initialise the command line arguments and directories.
fn init() -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation").expect("Failed to print title.");
    banner::sub_section("Command line arguments").expect("Failed to print title.");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    println!("{:>32} : {}", "binary path", bin_path.display());
    println!("{:>32} : {}", "parameters path", params_path.display());

    banner::sub_section("Directories").expect("Failed to print title.");
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");
    println!("{:>32} : {}", "input directory", in_dir.display());
    println!("{:>32} : {}", "output directory", out_dir.display());

    (params_path, in_dir, out_dir)
}
