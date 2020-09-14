//! Rendering engine binary.

use arctk::{
    args,
    err::Error,
    util::{banner, dir, exec, report},
};
use arctk_attr::input;
use palette::{Gradient, LinSrgba};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

fn main() {
    banner::title("RENDER").expect("Failed to print title.");
    let (params_path, in_dir, out_dir) = init().expect("Failed to initialise.");
}

/// Initialise the command line arguments and directories.
fn init() -> Result<(PathBuf, PathBuf, PathBuf), Error> {
    banner::section("Initialisation").expect("Failed to print title.");
    banner::sub_section("Command line arguments").expect("Failed to print title.");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    // report!("binary path", bin_path.display());
    // report!("parameters path", params_path.display());

    banner::sub_section("Directories").expect("Failed to print title.");
    let cwd = current_dir()?;
    let exec_name = exec::name()?;
    let (in_dir, out_dir) = dir::io_dirs(
        Some(cwd.join("input").join(exec_name.clone())),
        Some(cwd.join("output").join(exec_name)),
    )?;
    // report!("input directory", in_dir.display());
    // report!("output directory", out_dir.display());

    Ok((params_path, in_dir, out_dir))
}
