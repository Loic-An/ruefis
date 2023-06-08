extern crate alloc;
use core::{borrow::BorrowMut, fmt::Write};

use alloc::vec;
use alloc::vec::Vec;
use micromath::F32Ext;

use uefi::{
    table::{Boot, SystemTable},
    Status,
};

use crate::constants::{
    Const, BACKGROUND_ASCIICODE, CUBE_WIDTH, DISTANCE_FROM_CAM, HEIGHT, HORIZONTAL_OFFSET,
    INCREMENT_SPEED, K1, PIXELS, WIDTH,
};
pub struct Buffers {
    pub z_buffer: Vec<f32>,
    pub buffer: Vec<char>,
}
#[derive(Clone)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
    pub pixels: usize,
}
impl Copy for Resolution {}
fn get_resolution(stdout: &mut uefi::proto::console::text::Output) -> Resolution {
    if let Some(mode) = stdout.current_mode().unwrap() {
        return Resolution {
            width: mode.columns(),
            height: mode.rows(),
            pixels: mode.columns() * mode.rows(),
        };
    } else {
        return Resolution {
            width: WIDTH as usize,
            height: HEIGHT as usize,
            pixels: PIXELS,
        };
    }
}

fn update_const(o: Const) -> Const {
    let c = o.compteur + 0.02;
    return Const {
        cos_a: F32Ext::cos(c * 5.0),
        sin_a: F32Ext::sin(c * 5.0),
        cos_b: F32Ext::cos(c * 5.0),
        sin_b: F32Ext::sin(c * 5.0),
        cos_c: F32Ext::cos(c),
        sin_c: F32Ext::sin(c),
        compteur: c,
    };
}

pub fn run(mut st: SystemTable<Boot>) -> Status {
    st.stdout().enable_cursor(false).unwrap_or(());
    let res: Resolution = get_resolution(st.stdout().borrow_mut());
    let mut o = Const {
        cos_a: 1.0,
        sin_a: 0.0,
        cos_b: 1.0,
        sin_b: 0.0,
        cos_c: 1.0,
        sin_c: 0.0,
        compteur: 0.0,
    };
    const CW: f32 = CUBE_WIDTH as f32;
    st.stdout().clear();
    loop {
        let mut buffers = Buffers {
            z_buffer: vec![0.0; res.pixels],
            buffer: vec![BACKGROUND_ASCIICODE; res.pixels],
        };
        let mut cube_x = -CW;
        while cube_x < CW {
            let mut cube_y = -CW;
            while cube_y < CW {
                buffers = calculate_for_surface(buffers, cube_x, cube_y, -CW, o, res, '@');
                buffers = calculate_for_surface(buffers, CW, cube_y, cube_x, o, res, '$');
                buffers = calculate_for_surface(buffers, -CW, cube_y, -cube_x, o, res, '~');
                buffers = calculate_for_surface(buffers, -cube_x, cube_y, CW, o, res, '#');
                buffers = calculate_for_surface(buffers, cube_x, -CW, -cube_y, o, res, ';');
                buffers = calculate_for_surface(buffers, cube_x, CW, cube_y, o, res, '+');
                cube_y += INCREMENT_SPEED;
            }
            cube_x += INCREMENT_SPEED
        }
        st.stdout().set_cursor_position(0, 0);
        for c in 0..res.pixels {
            st.stdout().write_char(buffers.buffer[c]);
        }
        o = update_const(o);
    }
}
fn calculate_x(i: f32, j: f32, k: f32, o: Const) -> f32 {
    return j * (o.sin_a * o.sin_b * o.cos_c + o.cos_a * o.sin_c)
        - k * (o.cos_a * o.sin_b * o.cos_c + o.sin_a * o.sin_c)
        + i * o.cos_b * o.cos_c;
}

fn calculate_y(i: f32, j: f32, k: f32, o: Const) -> f32 {
    return j * (o.cos_a * o.cos_c - o.sin_a * o.sin_b * o.sin_c)
        + k * (o.sin_a * o.cos_c + o.cos_a * o.sin_b * o.sin_c)
        - i * o.cos_b * o.sin_c;
}

fn calculate_z(i: f32, j: f32, k: f32, o: Const) -> f32 {
    return k * o.cos_a * o.cos_b - j * o.sin_a * o.cos_b + i * o.sin_b;
}

fn calculate_for_surface(
    mut buffers: Buffers,
    cube_x: f32,
    cube_y: f32,
    cube_z: f32,
    o: Const,
    res: Resolution,
    ch: char,
) -> Buffers {
    let x = calculate_x(cube_x, cube_y, cube_z, o);
    let y = calculate_y(cube_x, cube_y, cube_z, o);
    let z = calculate_z(cube_x, cube_y, cube_z, o) + DISTANCE_FROM_CAM as f32;

    let ooz = 1.0 / z;

    let xp = (res.width as f32 / 2.0 + HORIZONTAL_OFFSET + K1 * ooz * x * 2.0) as u32;
    let yp = (res.height as f32 / 2.0 + K1 * ooz * y) as u32;

    let idx = xp + yp * res.width as u32;
    if idx < res.pixels as u32 {
        if ooz > buffers.z_buffer[idx as usize] {
            buffers.z_buffer[idx as usize] = ooz;
            buffers.buffer[idx as usize] = ch;
        }
    }
    buffers
}
