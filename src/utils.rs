pub fn halt_cpu() -> ! {
    unsafe {
        asm!("HLT");
    }

    loop {}; // Never reach here
}