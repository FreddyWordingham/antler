use antler::prelude::*;

fn main() {
    let resolution = [800, 600];
    let colour_a = Rgba::new(0.5, 0.25, 0.75, 0.5);
    let colour_b = Rgba::new(0.25, 0.75, 0.5, 0.5);

    let mut image = RgbaImage::filled(resolution, colour_a);

    for n in 0..resolution[0].min(resolution[1]) {
        image[(n, n)] = colour_b;
    }

    image.save("output.png").unwrap();
}
