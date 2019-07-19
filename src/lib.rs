#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga;
pub mod interrupts;
pub mod test_utils;

pub fn init() {
    interrupts::init_idt();
}


#[cfg(test)]
use core::panic::PanicInfo;

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_utils::test_panic_handler(info)
}
