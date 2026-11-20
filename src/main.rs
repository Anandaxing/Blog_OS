#![no_main]
#![no_std]
use core::panic::PanicInfo;

mod vga_buffer;

// Panic function returns nothing by using !
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

// static HELLO: &[u8] = b"Hello World!";

// Set attribute to disable name mangling to prevent the compiler to generate symbol to give every function a unique name
#[unsafe(no_mangle)]
// Extern "C": the compiler should use the C calling convention for this func
pub extern "C" fn _start() -> ! {
  // let vga_buffer = 0xb8000 as *mut u8;

  // for(i, &byte) in HELLO.iter().enumerate() {
  //   unsafe {
  //     *vga_buffer.offset(i as isize * 2) = byte;
  //     *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
  //   }
  // }
  // vga_buffer::print_something();

  use core::fmt::Write;
  vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
  write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
  
  loop {}
}
