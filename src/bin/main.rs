use std::fs::create_dir_all;

use antler::prelude::*;

fn main() {
    let built = SceneConfig::load("input/scene.toml").unwrap().build().unwrap();

    for (image_name, built_camera) in &built.cameras {
        for (render_name, render) in &built_camera.renders {
            let dir = format!("output/{image_name}");
            create_dir_all(&dir).unwrap();

            let path = format!("{dir}/{render_name}.png");
            let settings = RenderSettings::from_config(render, built_camera.background);
            let label = format!("{image_name}/{render_name}");

            let image = render_image_with_progress(
                &built.world,
                &built.scene,
                &built_camera.camera,
                settings,
                label,
                format!("rendered {path}"),
            );

            image.save(&path).unwrap();
            println!("saved {path}");
        }
    }
}
