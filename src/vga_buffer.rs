use volatile::Volatile;
use core::fmt;

#[allow(dead_code)] // Disable warning for each unused variant 
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // we enable copy semantics for the type and make it printable and comparable
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
#[repr(transparent)] // To ensure the ColorCode has the exact same data layout as u8, we use repr(transparent) attribute
struct ColorCode(u8);

impl ColorCode {
  fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // Field ordering in rust by default is undefined, by using C, it guarantees the correct field ordering
struct ScreenChar {
  ascii_character: u8,
  color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
  chars: [[Volatile<ScreenChar>; BUFFER_HEIGHT]; BUFFER_WIDTH],
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
          color_code 
        });
        self.column_position += 1;
      }
    }
  }

  pub fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
      match byte {
        // printable ASCII byte or newline
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        // not part of printable ASCII range
        _ => self.write_byte(0xfe),
      }
    }
  }

  fn clear_row(&mut self, row: usize) {
    let blank = ScreenChar {
      ascii_character: b' ',
      color_code: self.color_code,
    };
    for col in 0..BUFFER_WIDTH {
      self.buffer.chars[row][col].write(blank);
    }
  }
  
  fn new_line(&mut self) {
    for row in 1..BUFFER_HEIGHT {
      for col in 0..BUFFER_WIDTH {
        let character = self.buffer.chars[row][col].read();
        self.buffer.chars[row-1][col].write(character);
      }
    }
      self.clear_row(BUFFER_HEIGHT - 1);
      self.column_position = 0;
  }
}

// To support rust's formatting macros so that we can easily print different types like integers of floats.
impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);
    Ok(())
  }
}

pub fn print_something() {
  use core::fmt::Write;
  let mut writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
  };

  writer.write_byte(b'H');
  writer.write_string("ello dede");
  writer.write_string("koko");
  // The write! call returns a Result which causes a warning if not used, so we call the unwrap method on it, which panics if an error occurs. 
  write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}

use lazy_static::lazy_static;

lazy_static! {
  pub static ref WRITER: Writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  };
}