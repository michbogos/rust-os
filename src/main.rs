#![no_std]
#![no_main]

use core::{borrow::BorrowMut, panic::PanicInfo, fmt::Write};

use vga::VGABuffer;

mod vga;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!(*vga::VGA.lock(), "Hello {}\n", 1.2);
    // *vga_buffer.offset(i as isize * 2) = byte;
    // *vga_buffer.offset(i as isize * 2 + 1) = i as u8;

    loop {}
}
