#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Rgb {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);

    #[inline]
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        assert!(0.0 <= red && red <= 1.0, "Red value must be between 0.0 and 1.0.");
        assert!(0.0 <= green && green <= 1.0, "Green value must be between 0.0 and 1.0.");
        assert!(0.0 <= blue && blue <= 1.0, "Blue value must be between 0.0 and 1.0.");

        Self { red, green, blue }
    }
}
