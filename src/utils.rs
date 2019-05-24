pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

pub fn halt_cpu() -> ! {
    unsafe {
        asm!("hlt");
    }

    loop {} // Never reach here
}

pub fn out_byte(dst: u16, value: u8) {
    unsafe {
        asm!(
            "mov $0, %dx
            mov $1, %al
            out %al, %dx"
            : // no output
            : "r"(dst), "r"(value)
            : "al", "dx"
        );
    }
}

pub fn out_word(dst: u16, value: u16) {
    unsafe {
        asm!(
            "mov $0, %dx
            mov $1, %ax
            out %ax, %dx"
            : // no output
            : "r"(dst), "r"(value)
            : "ax", "dx"
        );
    }
}

pub fn out_dword(dst: u16, value: u32) {
    unsafe {
        asm!(
            "mov $0, %dx
            mov $1, %eax
            out %eax, %dx"
            : // no output
            : "r"(dst), "r"(value)
            : "eax", "dx"
        );
    }
}

pub fn disable_cursor() {
    out_byte(0x3D4, 0x0A);
    out_byte(0x3D5, 0x20);
}

pub fn move_cursor(x: usize, y: usize) {
    let position = y * BUFFER_WIDTH + x;
    out_byte(0x3D4, 0x0F);
    out_byte(0x3D5, (position & 0xFF) as _);
    out_byte(0x3D4, 0x0E);
    out_byte(0x3D5, ((position >> 8) & 0xFF) as _);
}
