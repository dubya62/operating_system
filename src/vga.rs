use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::{interrupts, port::Port};

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
        // SAFETY: VGA memory buffer starts at address 0xb8000 and is exactly
        // `SCREEN_WIDTH * SCREEN_HIGHT * 2` bytes long.  Since VgaChar is 2 bytes, we are
        // referencing the exact buffer and not a byte more.
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
            self.buffer[SCREEN_HEIGHT - 1].fill(BLANK_CHAR);
            self.row_position = SCREEN_HEIGHT - 1;
        }
    }

    fn clear(&mut self) {
        self.buffer.fill([BLANK_CHAR; SCREEN_WIDTH]);
        self.row_position = 0;
        self.column_position = 0;
    }
}

pub fn clear_screen() {
    WRITER.lock().clear();
}

pub fn set_colour(bg: Colour, fg: Colour) {
    WRITER.lock().set_colour(VgaColour::new(bg, fg));
}

pub fn get_colour() -> VgaColour {
    WRITER.lock().colour
}

pub fn move_cursor(x: u8, y: u8) {
    let pos = y as u16 * SCREEN_WIDTH as u16 + x as u16;
    let mut porta = Port::new(0x3d4);
    let mut portb = Port::new(0x3d5);
    unsafe {
        porta.write(0x0F_u8);
        portb.write((pos & 0xFF) as u8);
        porta.write(0x0E_u8);
        portb.write(((pos >> 8) & 0xFF) as u8);
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
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! print {
    ($fg: ident, $lit: literal $($args: tt)*) => {{
        let col = $crate::vga::get_colour();
        $crate::vga::set_colour(col.bg(), $crate::vga::Colour::$fg);
        print!($lit $($args)*);
        $crate::vga::set_colour(col.bg(), col.fg());
    }};
    ($lit: literal $($args: tt)*) => {
        ($crate::vga::_print(format_args!($lit $($args)*)))
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
        println!($lit $($args)*);
        $crate::vga::set_colour(col.bg(), col.fg());
    }};
    ($lit: literal $($args: tt)*) => {
        print!("{}\n", format_args!($lit $($args)*));
    };
}

#[macro_export]
macro_rules! dbg {
    () => {
        println!("[{}:{}:{}]", file!(), line!(), column!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                eprintln!("[{}:{}:{}] {} = {:#?}",
                    file!(), line!(), column!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(dbg!($val)),+,)
    };
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! print_one_line {
        ($line: literal) => {{
            assert!($line.len() < SCREEN_WIDTH); // Just ensure that it fits on one line
            println!("{}", $line);
            $line
        }};
    }

    macro_rules! assert_line_is {
        ($number: expr, $line: expr) => {
            let mut chars = $line.chars();
            for i in 0..SCREEN_WIDTH {
                let screen_char = WRITER.lock().buffer[$number][i];
                if let Some(c) = chars.next() {
                    assert_eq!(screen_char.char_code as char, c);
                } else {
                    assert_eq!(screen_char.char_code, 0);
                }
            }
        };
    }

    #[test_case]
    fn test_write() {
        clear_screen();
        let s = print_one_line!("Lorem ipsum dolor sit amet");
        assert_line_is!(0, s);
    }

    #[test_case]
    fn test_lines() {
        clear_screen();
        let s1 = print_one_line!("Lorem ipsum dolor sit amet");
        let s2 = print_one_line!("consectetur adipiscing elit");
        assert_line_is!(0, s1);
        assert_line_is!(1, s2);
    }

    #[test_case]
    fn test_many_lines() {
        clear_screen();
        print_one_line!("Lorem ipsum dolor sit amet");
        let s2 = print_one_line!("consectetur adipiscing elit");
        for n in 2..SCREEN_HEIGHT {
            println!("line {}", n);
        }
        assert_line_is!(0, s2);
    }
}
