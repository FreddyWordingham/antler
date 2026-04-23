use antler::prelude::*;

fn main() {
    std::fs::create_dir_all("output").unwrap();

    let built = SceneConfig::load("input/scene.toml").unwrap().build().unwrap();

    let image = render_image(
        &built.world,
        &built.scene,
        &built.camera,
        built.render.resolution,
        built.render.super_samples,
    );

    image.save("output/output.png").unwrap();
}
