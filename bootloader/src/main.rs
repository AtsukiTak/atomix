#![no_std]
#![no_main]

use core::fmt::Write;
use uefi::{
    table::{Boot, SystemTable},
    Handle, ResultExt as _, Status,
};

#[no_mangle]
extern "win64" fn efi_main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    writeln!(st.stdout(), "Hello, world!").unwrap();

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
