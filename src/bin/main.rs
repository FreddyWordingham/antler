use antler::prelude::*;

fn main() {
    // let manifest = Manifest::load("input/manifest.toml").unwrap();

    // println!("Manifest:");
    // println!("{:?}", manifest);

    let manifest = Manifest {
        width: 800,
        height: 600,
        background: Rgb::new(255.0, 255.0, 255.0),
    };
    manifest.save("input/manifest.toml").unwrap();
}
