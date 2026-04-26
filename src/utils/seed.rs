#[inline]
pub fn pixel_seed(coord: [usize; 2], sub_coord: [usize; 2]) -> u64 {
    let [x, y] = coord;
    let [sx, sy] = sub_coord;

    let mut z = x as u64;
    z ^= (y as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    z ^= (sx as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z ^= (sy as u64).wrapping_mul(0x94D0_49BB_1331_11EB);

    z ^= z >> 30;
    z = z.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z ^= z >> 27;
    z = z.wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
