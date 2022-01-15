#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use uefi::prelude::*;
use core::fmt::Write;

#[entry]
fn efi_main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    writeln!(st.stdout(), "Hello, world!").unwrap();

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
