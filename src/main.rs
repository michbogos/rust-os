#![no_std]
#![no_main]

use core::{borrow::BorrowMut, panic::PanicInfo, fmt::Write};

use vga::VGABuffer;

mod vga;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    write!(*vga::VGA.lock(), "{}", _info.message()).unwrap();
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!(*vga::VGA.lock(), "Hello {}\n", 1.2).unwrap();
    // *vga_buffer.offset(i as isize * 2) = byte;
    // *vga_buffer.offset(i as isize * 2 + 1) = i as u8;

    loop {}
}
