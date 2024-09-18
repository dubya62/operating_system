use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

const SCREEN_HEIGHT: usize = 25;
const SCREEN_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaChar {
    char_code: u8,
    color: u8,
}

const BLANK_CHAR: VgaChar = VgaChar {
    char_code: 0,
    color: 0,
};

#[repr(transparent)]
struct TextBuffer {
    chars: [[VgaChar; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    // color_code: ColorCode,
    buffer: &'static mut TextBuffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        // color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut TextBuffer) },
    });
}

impl Writer {
    pub fn write(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= SCREEN_WIDTH {
                    self.new_line();
                }

                let col = self.column_position;

                self.buffer.chars[self.row_position][col] = VgaChar {
                    char_code: byte,
                    color: 0xd,
                };
                self.column_position += 1;
            }
        }
    }

    pub fn write_string<S>(&mut self, s: S)
    where
        S: AsRef<str>,
    {
        for c in s.as_ref().chars() {
            if c.is_ascii() && (!c.is_ascii_control() || c == '\n') {
                self.write(c as u8)
            } else {
                self.write(0xfe)
            }
        }
        move_cursor(self.column_position as u8, self.row_position as u8);
    }

    fn new_line(&mut self) {
        self.row_position += 1;
        self.column_position = 0;
        if self.row_position > SCREEN_HEIGHT - 1 {
            self.buffer.chars.copy_within(1.., 0);
            self.buffer.chars[SCREEN_HEIGHT - 1] = [BLANK_CHAR; SCREEN_WIDTH];
            self.row_position = SCREEN_HEIGHT - 1;
        }
    }
}

pub fn move_cursor(x: u8, y: u8) {
    let pos = y as u16 * SCREEN_WIDTH as u16 + x as u16;
    unsafe {
        x86::io::outb(0x3D4, 0x0F);
        x86::io::outb(0x3D5, (pos & 0xFF) as u8);
        x86::io::outb(0x3D4, 0x0E);
        x86::io::outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($args: tt)*) => {
        ($crate::vga::_print(format_args!($($args)*)))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        print!("\n");
    };
    ($($args: tt)*) => {
        print!("{}\n", format_args!($($args)*));
    };
}
