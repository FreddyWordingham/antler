use std::{error::Error, fs::create_dir_all};

use antler::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest = Manifest::load("input/test.ron")?;
    let parameters = manifest.build();

    // Create output directory if it doesn't exist
    create_dir_all(&parameters.output_dir)?;

    for (scene_name, scene) in parameters.scenes {
        println!("> Scene: {scene_name}");
        for (capture_name, capture) in scene.captures {
            println!("  > Capture: {capture_name}");
            for (image_name, image) in capture.images {
                println!("    > Image: {image_name}");

                let image = render_image(
                    &image,
                    &parameters.settings,
                    &capture.camera,
                    &parameters.resources,
                    &scene.scene,
                );

                let image_path = parameters
                    .output_dir
                    .join(format!("{scene_name}-{capture_name}-{image_name}.png"));
                image.save(&image_path)?;
            }
        }
    }

    Ok(())
}
