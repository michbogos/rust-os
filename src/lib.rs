#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod serial;
pub mod vga;
pub mod interrupts;
pub mod gdt;
use core::panic::PanicInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(_info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", _info);
    exit_qemu(QemuExitCode::Failed);
    loop {x86_64::instructions::hlt()};
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {x86_64::instructions::hlt()};
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn init() {
    interrupts::init_idt();
    gdt::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}