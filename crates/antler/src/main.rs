use std::{error::Error, fs::create_dir_all};

use antler::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest = Manifest::load("input/test.ron")?;
    let parameters = manifest.build()?;

    // Create output directory if it doesn't exist
    create_dir_all(&parameters.output_dir)?;

    for (scene_name, scene) in parameters.scenes {
        println!("> Scene: {scene_name}");
        for (capture_name, capture) in scene.captures {
            println!("  > Capture: {capture_name}");
            for (render_name, render) in capture.renders {
                println!("    > Image: {render_name}");

                let (visual_image, temporal_image) =
                    render_image(&render, &capture.camera, &parameters.resources, &scene.scene);

                let visual_image_path = parameters
                    .output_dir
                    .join(format!("{scene_name}-{capture_name}-{render_name}.png"));
                // visual_image.tone_map();
                visual_image.save(&visual_image_path)?;

                let temporal_image_path = parameters
                    .output_dir
                    .join(format!("{scene_name}-{capture_name}-{render_name}-temporal.png"));
                temporal_image.save(&temporal_image_path)?;
            }
        }
    }

    Ok(())
}
