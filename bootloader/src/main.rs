#![no_std]
#![no_main]
#![feature(once_cell)]

mod uefi;
mod console;

use self::uefi::{proto::fs::SimpleFileSystemProtocol, Handle, SystemTable};

#[no_mangle]
extern "win64" fn efi_main(handle: Handle, table: SystemTable) -> usize {
    console::text::output::set_stdout(table.console_out());
    console::text::output::clear();

    println!("Hello World");

    let mut buf = [0u8; 8000];
    match table.boot().get_memory_map(&mut buf) {
        Ok((_key, mem_map_iter)) => {
            for mem_desc in mem_map_iter {
                println!("{:?}", mem_desc);
            }
        }
        Err(size) => {
            println!("required memory map size : {}", size);
        }
    };

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop {}
}
