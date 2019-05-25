use core::fmt::Write;
use spin::Mutex;
use volatile::Volatile;

use crate::utils::{self, BUFFER_HEIGHT, BUFFER_WIDTH};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode(foreground as u8 | ((background as u8) << 4))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color: ColorCode,
}

impl ScreenChar {
    fn new(b: u8, color: ColorCode) -> ScreenChar {
        ScreenChar {
            ascii_char: b,
            color: color,
        }
    }
}

#[repr(C)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    row: usize,
    column: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::vga_buffer(ColorCode::new(
        Color::Black,
        Color::LightGray
    )));
}

impl Writer {
    pub fn vga_buffer(color: ColorCode) -> Writer {
        let mut writer = Writer {
            row: 0,
            column: 0,
            color: color,
            buffer: unsafe { &mut *(0xb8000 as *mut _) },
        };

        for i in 0..BUFFER_HEIGHT {
            writer.clear_row(i);
        }

        writer
    }

    pub fn clear_row(&mut self, row: usize) {
        for i in 0..BUFFER_WIDTH {
            self.buffer.chars[row][i].write(ScreenChar::new(b' ', self.color));
        }
    }

    fn move_up(&mut self) {
        for i in 0..(BUFFER_HEIGHT - 1) {
            self.clear_row(i);
            for j in 0..BUFFER_WIDTH {
                self.buffer.chars[i][j].write(self.buffer.chars[i + 1][j].read());
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
    }

    pub fn new_line(&mut self) {
        self.row += 1;
        if self.row >= BUFFER_HEIGHT {
            self.row = BUFFER_HEIGHT - 1;
            self.move_up();
        }
        self.column = 0;
    }

    pub fn write_byte(&mut self, b: u8) {
        if b == b'\n' {
            self.new_line();
            utils::move_cursor(self.column, self.row);
            return;
        }

        self.buffer.chars[self.row][self.column].write(ScreenChar::new(b, self.color));

        self.column += 1;

        if self.column == BUFFER_WIDTH {
            self.new_line();
        }

        utils::move_cursor(self.column, self.row);
    }

    pub fn write_string(&mut self, cs: &str) {
        for b in cs.as_bytes().iter() {
            match b {
                0x20...0x7e | b'\n' => self.write_byte(*b),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
