#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ros::{init, println, utils};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    println!("Hello, world!");
    x86_64::instructions::interrupts::int3();
    println!("こんにちは、世界!");

    #[cfg(test)]
    {
        test_main();
    }
    utils::halt_cpu()
}
