use antler::prelude::*;
use chromatic::{Colour, ColourMap, HslAlpha};
use nalgebra::{Point3, RealField};
use ndarray::Array2;
use photo::Image;
use std::path::Path;

type P = f32;

const COLOURS: [&str; 2] = ["#000000FF", "#FFFFFFFF"];

fn main() {
    let cmap = ColourMap::new_uniform(
        COLOURS
            .iter()
            .map(|&c| HslAlpha::<P>::from_hex(c).unwrap())
            .collect::<Vec<_>>()
            .as_slice(),
    );

    let camera = Camera::new(
        Point3::new(10.0, 10.0, 10.0),
        Point3::new(0.0, 0.0, 3.0),
        90.0_f32.to_radians(),
        [600, 800],
    );
    let mesh = Mesh::<P>::load(Path::new("./assets/meshes/tree.obj"));
    println!("Max BVH depth: {}", mesh.bvh_depth());

    let mut distance = Array2::from_elem([600, 800], 0.0);

    for row in 0..600 {
        println!("Processing row {}/600", row);
        for col in 0..800 {
            let ray = camera.generate_ray([row, col]);
            if let Some(intersection) = mesh.intersect(&ray) {
                distance[[row, col]] = intersection.distance;
            }
        }
    }

    let (min, max) = distance
        .iter()
        .fold((P::max_value().unwrap(), P::min_value().unwrap()), |(min, max), &val| {
            (min.min(val), max.max(val))
        });
    println!("Min value: {}, Max value: {}", min, max);
    distance.par_mapv_inplace(|x| (x - min) / (max - min));

    let img = distance.mapv(|x| cmap.sample(x));
    img.save(Path::new("./output/image.png")).unwrap();
}
