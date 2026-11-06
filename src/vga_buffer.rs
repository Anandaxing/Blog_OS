#[allow(dead_code)] // Disable warning for each unused variant 
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // we enable copy semantics for the type and make it printable and comparable
#[repr(transparent)] // To ensure the ColorCode has the exact same data layout as u8, we use repr(transparent) attribute
struct ColorCode(u8);

impl ColorCode {
  fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
  }
}

pub struct Writer {
  column_position: usize,
  color_code: ColorCode,
  buffer: &'static mut Buffer,
}

#[repr(C)] // Field ordering in rust by default is undefined, by using C, it guarantees the correct field ordering
struct ScreenChar {
  ascii_character: u8,
  color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
  chars: [[ScreenChar; BUFFER_HEIGHT]; BUFFER_WIDTH],
}


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