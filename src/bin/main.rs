use std::fs::create_dir_all;

use antler::prelude::*;

fn main() {
    let built = SceneConfig::load("input/scene.toml").unwrap().build().unwrap();

    for (camera_name, built_camera) in &built.cameras {
        for (render_name, render) in &built_camera.renders {
            let dir = format!("output/{camera_name}");
            create_dir_all(&dir).unwrap();

            let path = format!("{dir}/{render_name}.png");
            let settings = RenderSettings::from(render);

            let image = render_image(&built.world, &built.scene, &built_camera.camera, settings);

            image.save(path).unwrap();
        }
    }
}
