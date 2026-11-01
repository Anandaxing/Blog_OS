#![no_main]
#![no_std]
use core::panic::PanicInfo;

// Panic function returns nothing by using !
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

// Set attribute to disable name mangling to prevent the compiler to generate symbol to give every function a unique name
#[unsafe(no_mangle)]
// Extern "C": the compiler should use the C calling convention for this func
pub extern "C" fn _start() -> ! {
  loop {}
}
