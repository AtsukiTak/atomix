use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Write a string to the VGA buffer.
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = BottomWriter::new(ColorCode::new(Color::Yellow, Color::Black));
    writer.write_fmt(args).unwrap();
}

struct BottomWriter {
    column_position: usize,
    color_code: ColorCode,
}

impl core::fmt::Write for BottomWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
        Ok(())
    }
}

impl BottomWriter {
    fn new(color_code: ColorCode) -> BottomWriter {
        BottomWriter {
            column_position: 0,
            color_code,
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUF_WIDTH {
                    self.new_line();
                }
                let row = BUF_HEIGHT - 1;
                let col = self.column_position;
                let screen_char = ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                };
                write_char(screen_char, row, col);

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUF_HEIGHT {
            for col in 0..BUF_WIDTH {
                let c = read_char(row, col);
                write_char(c, row - 1, col);
            }
        }
        self.clear_row(BUF_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUF_WIDTH {
            write_char(blank, row, col);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Color {
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

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((foreground as u8) | (background as u8) << 4)
    }
}

const BUF_HEIGHT: usize = 25;
const BUF_WIDTH: usize = 80;

lazy_static! {
    static ref VGA_BUFFER: Mutex<&'static mut [[Volatile<ScreenChar>; BUF_WIDTH]; BUF_HEIGHT]> =
        Mutex::new(unsafe { &mut *(0xb8000 as *mut _) });
}

/// Write a character to the VGA buffer.
fn write_char(screen_char: ScreenChar, row: usize, col: usize) {
    VGA_BUFFER.lock()[row][col].write(screen_char);
}

/// Read a character from the VGA buffer.
fn read_char(row: usize, col: usize) -> ScreenChar {
    VGA_BUFFER.lock()[row][col].read()
}
