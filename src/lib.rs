#![no_std]
#![cfg_attr(test, no_main)]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate spin;

pub mod interrupts;
pub mod utils;
pub mod vga_buffer;

#[cfg(test)]
mod serial;

use core::panic::PanicInfo;

pub fn init() {
    interrupts::init_idt();
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

// our existing panic handler
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    utils::out_word(QEMU_PORT, QemuExitCode::Failed as _);
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

const QEMU_PORT: u16 = 0xf4;

#[allow(unused_variables)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    #[cfg(test)]
    {
        serial_println!("Running {} tests", tests.len());
        for test in tests {
            test();
        }
    }

    utils::out_word(QEMU_PORT, QemuExitCode::Success as _);
}

#[test_case]
fn trivial_test() {
    serial_print!("A trivial test... ");
    assert_eq!(1, 1);
    serial_println!("done");
}
