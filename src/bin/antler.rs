//! Rendering engine binary.

use antler::{
    input::Scene,
    input::{Settings, Shader, ShaderBuilder},
    parts::Attributes,
};
use arctk::{
    args,
    err::Error,
    file::{Build, Load},
    geom::Tree,
    geom::{Mesh, MeshBuilder, TreeBuilder},
    img::GradientBuilder,
    ord::Set,
    util::{banner, dir, exec, gradient},
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
    /// Render runtime settings.
    sett: Settings,
    /// Surfaces map.
    surfs: Set<Key, MeshBuilder>,
    /// Attributes map.
    attrs: Set<Key, Attributes>,
    /// Colour map.
    cols: Set<Key, GradientBuilder>,
    /// Shader.
    shader: ShaderBuilder,
}

fn main() {
    banner::title("RENDER").expect("Failed to print title.");
    let (params_path, in_dir, out_dir) = init();
    let params = input(&in_dir, &params_path);
    let (tree_sett, render_sett, surfs, attrs, cols, shader) = build(&in_dir, params);
    let tree = grow(tree_sett, &surfs);
    let input = Scene::new(&tree, &render_sett, &surfs, &attrs, &cols);
}

/// Initialise the command line arguments and directories.
fn init() -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation").expect("Failed to print section heading.");
    banner::sub_section("Command line arguments").expect("Failed to print sub-section heading.");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    println!("{:>32} : {}", "binary path", bin_path.display());
    println!("{:>32} : {}", "parameters path", params_path.display());

    banner::sub_section("Directories").expect("Failed to print sub-section heading.");
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");
    println!("{:>32} : {}", "input directory", in_dir.display());
    println!("{:>32} : {}", "output directory", out_dir.display());

    (params_path, in_dir, out_dir)
}

/// Load the input files.
fn input(in_dir: &Path, params_path: &Path) -> Parameters {
    banner::section("Input").expect("Failed to print section heading.");
    banner::sub_section("Parameters").expect("Failed to print sub-section heading.");
    let path = in_dir.join(params_path);

    Parameters::load(&path).expect("Failed to load parameters file.")
}

/// Build instances.
#[allow(clippy::type_complexity)]
fn build(
    in_dir: &Path,
    params: Parameters,
) -> (
    TreeBuilder,
    Settings,
    Set<Key, Mesh>,
    Set<Key, Attributes>,
    Set<Key, Gradient<LinSrgba>>,
    Shader,
) {
    banner::section("Building").expect("Failed to print section heading.");
    banner::sub_section("Adaptive Tree Settings").expect("Failed to print sub-section heading.");
    let tree_sett = params.tree;
    // println!("{:>32} : {}", "Tree settings", tree_sett);

    banner::sub_section("Render Settings").expect("Failed to print sub-section heading.");
    let render_sett = params.sett;
    // println!("{:>32} : {}", "Render settings", render_sett);

    banner::sub_section("Surfaces").expect("Failed to print sub-section heading.");
    let surfs = params
        .surfs
        .build(in_dir)
        .expect("Unable to build surfaces.");
    // println!("{:>32} : {}", "Surfaces", surfs);

    banner::sub_section("Attributes").expect("Failed to print sub-section heading.");
    let attrs = params.attrs;
    // println!("{:>32} : {}", "Attributes", attrs);

    banner::sub_section("Colours").expect("Failed to print sub-section heading.");
    let cols = params
        .cols
        .build(in_dir)
        .expect("Unable to build colour gradients.");
    for (group, grad) in cols.map() {
        println!(
            "{:>32} : {}",
            &format!("[{}]", group),
            gradient::to_string(&grad, 32)
        );
    }

    banner::sub_section("Shaders").expect("Failed to print sub-section heading.");
    let shader = params
        .shader
        .build(in_dir)
        .expect("Unable to build scenes.");
    // println!("{:>32} : {}", "Main image", shader);

    (tree_sett, render_sett, surfs, attrs, cols, shader)
}

/// Grow domains.
fn grow<'a>(tree: TreeBuilder, surfs: &'a Set<Key, Mesh>) -> Tree<'a, &Key> {
    banner::section("Growing").expect("Failed to print section heading.");

    banner::sub_section("Adaptive Tree").expect("Failed to print sub-section heading.");
    let tree = tree.build(&surfs);
    // println!("{:>32} : {}", "Adaptive tree", &tree);

    tree
}
