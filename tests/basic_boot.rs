#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(atomix::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

use atomix::{println, serial_print, serial_println};
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
    serial_println!("[ok]");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    atomix::test_utils::test_panic_handler(info);
}
