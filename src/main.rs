#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use raw_cpuid::CpuId;

mod vga;

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[test_case]
fn get_cpuid() {
    print!("Getting cpuid... ");
    let cpuid = CpuId::new();
    if let Some(vf) = cpuid.get_vendor_info() {
        println!("{}", vf.as_str());
        assert!(vf.as_str() == "GenuineIntel" || vf.as_str() == "AuthenticAMD");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}


/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info.message());
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}\n", 1.2);
    #[cfg(test)]
    test_main();
    // *vga_buffer.offset(i as isize * 2) = byte;
    // *vga_buffer.offset(i as isize * 2 + 1) = i as u8;

    loop {}
}