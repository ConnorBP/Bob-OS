/// "Safe" wrapper for the VGA ASCII Output Buffer

use core::fmt;
use spin::Mutex;
use volatile::Volatile;
use lazy_static::lazy_static;


/* Instead of computing its value at compile time,the static laziliy initializes itself when it's accessed the first time.
Thus, the initialization happens at runtime so that arbitrarily complex initialization code is possible. */
lazy_static! {
    pub static ref WRITER: Writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
}

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
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}




impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20...0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }

    /// Shifts all lines one line up and clears the last row.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clears a row by overwriting it with blank characters.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

//#[derive(Copy, Clone)]
//pub struct Arguments<'a> {
//    // Format string pieces to print.
//    pieces: &'a [&'a str],
//
//    // Placeholder specs, or `None` if all specs are default (as in "{}{}").
//    fmt: Option<&'a [fmt::rt::v1::Argument]>,
//
//    // Dynamic arguments for interpolation, to be interleaved with string
//    // pieces. (Every argument is preceded by a string piece.)
//    args: &'a [fmt::ArgumentV1<'a>],
//}

//#[allow(missing_debug_implementations)]
//#[stable(feature = "rust1", since = "1.0.0")]
//pub struct Formatter<'a> {
//    flags: u32,
//    fill: char,
//    align: fmt::rt::v1::Alignment,
//    width: Option<usize>,
//    precision: Option<usize>,
//
//    buf: &'a mut (dyn fmt::Write+'a),
//    curarg: fmt::slice::Iter<'a, fmt::ArgumentV1<'a>>,
//    args: &'a [fmt::ArgumentV1<'a>],
//}

//impl fmt::Write for Writer {
//    fn write_str(&mut self, s: &str) -> fmt::Result {
//        self.write_string(s);
//        Ok(())
//    }
//
////    fn write_str(&mut self, _s: &str) -> core::fmt::Result {
////        unimplemented!();
////    }
//
//    fn write_fmt(&mut self, args: Arguments<'_>) -> fmt::Result {
//        unimplemented!();
//        //write(self.buffer, args);
//        //Ok(())
//    }
//
//}

//pub fn write(output: &mut dyn fmt::Write, args: Arguments<'_>) -> fmt::Result {
//    let mut formatter = Formatter {
//        flags: 0,
//        width: None,
//        precision: None,
//        buf: output,
//        align: fmt::rt::v1::Alignment::Unknown,
//        fill: ' ',
//        args: args.args,
//        curarg: args.args.iter(),
//    };
//
//    let mut idx = 0;
//
//    match args.fmt {
//        None => {
//            // We can use default formatting parameters for all arguments.
//            for (arg, piece) in args.args.iter().zip(args.pieces.iter()) {
//                formatter.buf.write_str(*piece)?;
//                (arg.formatter)(arg.value, &mut formatter)?;
//                idx += 1;
//            }
//        }
//        Some(fmt) => {
//            // Every spec has a corresponding argument that is preceded by
//            // a string piece.
//            for (arg, piece) in fmt.iter().zip(args.pieces.iter()) {
//                formatter.buf.write_str(*piece)?;
//                formatter.run(arg)?;
//                idx += 1;
//            }
//        }
//    }
//
//    // There can be only one trailing string piece left.
//    if let Some(piece) = args.pieces.get(idx) {
//        formatter.buf.write_str(*piece)?;
//    }
//
//    Ok(())
//}

/// Print to screen helper function (unsafe)
pub fn print_to_screen() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello!@ ");
    writer.write_string("Wörld!");
    //write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
    //write!(&mut writer, "Hello World");
}


//macro_rules! print {
//     ($($arg:tt)*) => ({
//            $crate::vga_buffer::print(format_args!($($arg)*));
//     });
//}
//
//pub fn print(args: fmt::Arguments) {
//    use core::fmt::Write;
//    WRITER.lock().write_fmt(args).unwrap();
//}