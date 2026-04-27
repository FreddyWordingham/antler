use antler::prelude::*;

fn main() {
    let width = 64;
    let height = 32;
    let colour = Rgba::new(0.9, 0.1, 0.1, 1.0);

    let mut image = RgbaImage::filled([width, height], Rgba::TRANSPARENT);

    for i in 0..width.min(height) {
        image[(i, i)] = colour;
    }

    image.save("output.png").expect("failed to save image");
}
