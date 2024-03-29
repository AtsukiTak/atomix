#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(atomix::test_utils::test_runner)]
#![reexport_test_harness_main = "test_main"]

use atomix::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    atomix::init();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!!");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    atomix::test_utils::test_panic_handler(info)
}
