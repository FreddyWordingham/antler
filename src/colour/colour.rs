use palette::Srgba;

pub struct Colour(Srgba);

impl Colour {
    pub const BLACK: Colour = Colour(Srgba::new(0.0, 0.0, 0.0, 1.0));
}
