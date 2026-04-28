use std::error::Error;

use antler::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest = Manifest::load_resolved("input/test.ron")?;
    println!("{:#?}", manifest);
    Ok(())
}
