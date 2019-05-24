#![no_std]
#![no_main]
#![feature(asm)]

extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate spin;

mod utils;
mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::WRITER;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    println!("こんにちは、世界!");
    println!("drink 🍺!");
    for i in 0..23 {
        println!("i: {}", i);
    }

    utils::halt_cpu()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
