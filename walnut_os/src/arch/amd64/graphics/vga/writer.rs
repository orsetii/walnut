use crate::util::sync::SpinLock;

use super::{
    char::ScreenChar,
    colours::{Color, ColorCode},
    Buffer, BUFFER_HEIGHT, BUFFER_WIDTH,
};

// TODO FIXME this probably should be inside
// a sync primitive that doesnt require it sits as a `static mut` lol
pub static WRITER: SpinLock<Writer> = SpinLock::new(Writer::new());

pub struct Writer {
    pub column_position: usize,
    color_code: ColorCode,
    buffer_addr: usize,
}

impl Writer {
    pub const fn new() -> Self {
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            buffer_addr: 0xb8000,
        }
    }

    fn buffer(&self) -> &'static mut Buffer {
        unsafe { &mut *(self.buffer_addr as *mut Buffer) }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= super::BUFFER_WIDTH {
                    self.new_line();
                }

                let row = super::BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                // Perform a volatile write on calculated MMIO address
                // of the char
                self.write_char(
                    row,
                    col,
                    ScreenChar {
                        ascii_character: byte,
                        color_code,
                    },
                );

                self.column_position += 1;
            }
        }
    }

    #[inline]
    fn write_char(&mut self, row: usize, col: usize, char: ScreenChar) {
        unsafe {
            self.buffer().chars[row]
                .as_mut_ptr()
                .add(col)
                .write_volatile(char)
        }
    }

    #[allow(dead_code)]
    #[inline]
    fn read_char(&mut self, row: usize, col: usize) -> ScreenChar {
        unsafe {
            self.buffer().chars[row]
                .as_mut_ptr()
                .add(col)
                .read_volatile()
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer().chars[row][col];
                self.write_char(row - 1, col, char);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.write_char(row, col, blank);
        }
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! vga_println {
    () => (print!("\n"));
    ($($arg:tt)*) => ($crate::arch::amd64::graphics::vga::_print(format_args!("{}\n", format_args!($($arg)*))));
}

#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::arch::amd64::graphics::vga::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[test_case]
fn test_println_many() {
    // prints every ascii char, 256 times
    for _ in 0..255 {
        crate::vga_println!(" !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{{}}~'");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    crate::vga_println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().read_char(BUFFER_HEIGHT - 2, i);
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
