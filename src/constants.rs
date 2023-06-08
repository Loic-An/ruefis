pub const CUBE_WIDTH: u32 = 10;
pub const WIDTH: u32 = 160;
pub const HEIGHT: u32 = 44;
pub const PIXELS: usize = (WIDTH * HEIGHT) as usize;
pub const BACKGROUND_ASCIICODE: char = ' '; // ' ' as u32
pub const DISTANCE_FROM_CAM: u32 = 100;
pub const HORIZONTAL_OFFSET: f32 = -2.0 * CUBE_WIDTH as f32;
pub const K1: f32 = 40.0;
pub const INCREMENT_SPEED: f32 = 0.6;

#[derive(Clone)]
pub struct Const {
    pub cos_a: f32,
    pub sin_a: f32,
    pub cos_b: f32,
    pub sin_b: f32,
    pub cos_c: f32,
    pub sin_c: f32,
    pub compteur: f32,
}
impl Copy for Const {}
