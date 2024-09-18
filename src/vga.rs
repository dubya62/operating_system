use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

const SCREEN_HEIGHT: usize = 25;
const SCREEN_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaChar {
    char_code: u8,
    color: VgaColour,
}

const BLANK_CHAR: VgaChar = VgaChar {
    char_code: 0,
    color: VgaColour::new(Colour::Black, Colour::White),
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
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
pub struct VgaColour(u8);

impl VgaColour {
    pub const fn new(bg: Colour, fg: Colour) -> Self {
        Self((bg as u8) << 4 | fg as u8)
    }

    pub const fn bg(&self) -> Colour {
        // SAFETY:  This is okay because 0 <= (u8 >> 4) <= 15, like this enum
        unsafe { core::mem::transmute(self.0 >> 4) }
    }

    pub const fn fg(&self) -> Colour {
        // SAFETY:  This is okay because 0 <= (u8 & 0xf) <= 15, like this enum
        unsafe { core::mem::transmute(self.0 & 0xf) }
    }
}

type TextBuffer = [[VgaChar; SCREEN_WIDTH]; SCREEN_HEIGHT];

pub struct Writer {
    column_position: usize,
    row_position: usize,
    colour: VgaColour,
    buffer: &'static mut TextBuffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        colour: VgaColour::new(Colour::Black, Colour::White),
        buffer: unsafe { &mut *(0xb8000 as *mut TextBuffer) },
    });
}

impl Writer {
    pub fn set_colour(&mut self, colour: VgaColour) {
        self.colour = colour;
    }

    pub fn write(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= SCREEN_WIDTH {
                    self.new_line();
                }

                self.buffer[self.row_position][self.column_position] = VgaChar {
                    char_code: byte,
                    color: self.colour,
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
            self.buffer.copy_within(1.., 0);
            self.buffer[SCREEN_HEIGHT - 1] = [BLANK_CHAR; SCREEN_WIDTH];
            self.row_position = SCREEN_HEIGHT - 1;
        }
    }
}

pub fn set_colour(bg: Colour, fg: Colour) {
    WRITER.lock().set_colour(VgaColour::new(bg, fg));
}

pub fn get_colour() -> VgaColour {
    WRITER.lock().colour
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
    ($fg: ident, $lit: literal $($args: tt)*) => {{
        let col = $crate::vga::get_colour();
        $crate::vga::set_colour(col.bg(), $crate::vga::Colour::$fg);
        print!("{}\n", format_args!($lit $($args)*));
        $crate::vga::set_colour(col.bg(), col.fg());
    }};
    ($lit: literal $($args: tt)*) => {
        print!("{}\n", format_args!($lit $($args)*));
    };
}
