use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Context {
    pub base_dir: PathBuf,
    pub assets_dir: PathBuf,
    pub output_dir: PathBuf,
}
