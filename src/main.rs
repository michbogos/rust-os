#![no_std]
#![no_main]

use core::panic::PanicInfo;

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
    let mut buffer:VGABuffer = VGABuffer{size:25*80,idx:0,adress:0xb8000 as *mut u16};
    vga::print(&mut buffer, "Hello", vga::VGACol::L_CYAN, vga::VGACol::GREEN);
    // *vga_buffer.offset(i as isize * 2) = byte;
    // *vga_buffer.offset(i as isize * 2 + 1) = i as u8;

    loop {}
}
