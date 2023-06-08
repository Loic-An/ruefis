#![no_std]
#![no_main]

use uefi::{
    entry,
    prelude::BootServices,
    table::{runtime::ResetType, Boot, SystemTable},
    Handle, Status,
};
use uefi_services::init;
mod app;
pub mod constants;

#[entry]
fn efi_main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    init(&mut st).unwrap();

    // Retrieve the graphics output protocol
    let _boot: &BootServices = st.boot_services();
    /*
    let gop_handle = boot.get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = boot
        .open_protocol_exclusive::<GraphicsOutput>(gop_handle)
        .unwrap();

    // Set video mode to the largest available resolution
    let max_mode = gop.modes().len() as u32;
    let mut mode_info = None;
    for mode_number in 0..max_mode {
        if let Ok(info) = gop.query_mode(mode_number) {
            mode_info = Some(info);
            break;
        }
    }

    // Switch to the selected video mode
    gop.set_mode(&mode_info.unwrap()).unwrap();

    //create a white colored pixel
    let white: PixelBitmask = PixelBitmask {
        red: 255,
        green: 255,
        blue: 255,
        reserved: 0,
    };
    let _mode = gop.current_mode_info();

    // Get the frame buffer base address and size
    // and fill the screen in white
    let mut frame_buffer: FrameBuffer = gop.frame_buffer();
    for i in 0..frame_buffer.size() {
        if let Some(addr) = unsafe {
            frame_buffer
                .as_mut_ptr()
                .add(i)
                .cast::<PixelBitmask>()
                .as_mut()
        } {
            *addr = white
        }
    }
    // return Status::SUCCESS;
    // app is running
    */
    app::run(st);
    panic!("EOF");
    //poweroff(st)
}

fn _poweroff(st: SystemTable<Boot>) -> ! {
    unsafe { st.exit_boot_services().0.runtime_services() }.reset(
        ResetType::SHUTDOWN,
        Status::SUCCESS,
        None,
    )
}
