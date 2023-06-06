#![no_std]
#![no_main]

use core::{panic::PanicInfo,panic};
use uefi::{Handle,Status,table::{Boot,SystemTable},prelude::BootServices,proto::console::gop::{PixelBitmask,GraphicsOutput}};
mod app;

#[panic_handler]
fn panic_handler(info:&PanicInfo)-> ! {
    panic!("{}",info)
}


#[export_name = "efi_main"]
fn efi_main(handle: Handle, system_table: &'static SystemTable<Boot>) -> Status {
    // Retrieve the graphics output protocol
    let boot: &BootServices = system_table.boot_services();
    let mut gop = match boot.open_protocol_exclusive::<GraphicsOutput>(handle) {
        Ok(protocol) => protocol,
        Err(e) => {
            // Handle the case when the protocol is not found
            return e.status();
        }
    };

    // Set video mode to the largest available resolution
    let max_mode = gop.modes().len() as u32;
    let mut mode_info = None;
    for mode_number in 0..max_mode {
        if let Ok(info) = gop.query_mode(mode_number) {
            mode_info = Some(info);
            break;
        }
    }

    if mode_info.is_none() {
        // Handle the case when no suitable video mode is found
        return Status::NOT_FOUND
    }

    // Switch to the selected video mode
    let active_mode = mode_info.unwrap();
    match gop.set_mode(&active_mode) {
        Ok(()) => {},
        Err(e) => return e.status()
    }

    // Get the frame buffer base address and size
    let mut frame_buffer = gop.frame_buffer();
    let frame_buffer_base = frame_buffer.as_mut_ptr();

    // Clear the screen to black
    let i=0;
    while i<frame_buffer.size() {
        unsafe {frame_buffer.write_byte(i, 0) }
    }

    //for i in 0..frame_buffer_size {
    //    if let Ok(addr) = unsafe { frame_buffer_base.add(i).as_mut() } {
    //        *addr = pixel;
    //    }
    //}

    // Draw a white rectangle in the center of the screen
    let (width,height) = active_mode.info().resolution();
    let rect_width = width / 4;
    let rect_height = height / 4;
    let rect_start_x = (width - rect_width) / 2;
    let rect_start_y = (height - rect_height) / 2;

    let white_pixel = PixelBitmask {
        blue: 255,
        green: 255,
        red: 255,
        reserved: 0,
    };

    for y in rect_start_y..(rect_start_y + rect_height) {
        for x in rect_start_x..(rect_start_x + rect_width) {
            if let Some(addr) = unsafe { frame_buffer_base.add(y * width + x).cast::<PixelBitmask>().as_mut() } {
                *addr = white_pixel;
            }
        }
    }
    return match app::run(system_table) {
        Ok(_) => Status::SUCCESS,
        Err(_) => Status(1234567890)
    };
}