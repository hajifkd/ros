#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(core_intrinsics)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ros::{init, println, utils};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    println!("Hello, world!");
    // x86_64::instructions::interrupts::int3();
    unsafe {
        let i = core::intrinsics::unchecked_div(1, 0);
        println!("Zero division result: {}", i);
    }
    x86_64::instructions::interrupts::int3();
    println!("こんにちは、世界!");

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();
    #[cfg(test)]
    {
        test_main();
    }
    utils::halt_cpu()
}
