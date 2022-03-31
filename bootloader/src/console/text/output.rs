use crate::uefi::proto::text::SimpleTextOutputProtocol;
use core::fmt;
use spin::{Lazy, Mutex};

static STDOUT: Lazy<Mutex<Option<Output>>> = Lazy::new(|| Mutex::new(None));

pub fn set_stdout(stdout: &'static SimpleTextOutputProtocol) {
    let writer = Output { stdout };
    *STDOUT.lock() = Some(writer);
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => ($crate::console::text::output::_print(format_args!($($args)*)))
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use fmt::Write;

    if let Some(output) = &mut *STDOUT.lock() {
        output.write_fmt(args).unwrap();
    }
}

pub struct Output {
    stdout: &'static SimpleTextOutputProtocol,
}

impl fmt::Write for Output {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buf = WriteBuf::new(self.stdout);
        buf.write_str(s);
        Ok(())
    }
}

struct WriteBuf {
    stdout: &'static SimpleTextOutputProtocol,
    buf: [u16; 128],
    i: usize,
}

impl WriteBuf {
    fn new(stdout: &'static SimpleTextOutputProtocol) -> Self {
        WriteBuf {
            stdout,
            buf: [0; 128],
            i: 0,
        }
    }

    fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
        self.flush();
    }

    fn write_char(&mut self, c: char) {
        // UEFIでサポートしているのはUCS2の範囲のみなので、
        // UTF16表現では必ず長さが1になる。
        assert!(c.len_utf16() == 1);

        // 最後のnull terminatorの分と、最大2文字分の余裕がなければflushする
        if self.i >= 125 {
            self.flush();
        }

        if c == '\n' {
            '\r'.encode_utf16(&mut self.buf[self.i..]);
            '\n'.encode_utf16(&mut self.buf[self.i + 1..]);
            self.i += 2;
        } else {
            c.encode_utf16(&mut self.buf[self.i..]);
            self.i += 1;
        }
    }

    fn flush(&mut self) {
        self.stdout.output_string(self.buf.as_ptr());
        self.buf = [0; 128];
        self.i = 0;
    }
}
