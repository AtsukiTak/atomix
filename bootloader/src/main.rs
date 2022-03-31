#![no_std]
#![no_main]
#![feature(once_cell)]

mod console;
mod uefi;

use self::uefi::{
    proto::{file::OpenMode, fs::SimpleFileSystemProtocol},
    Handle, SystemTable,
};
use core::fmt::Write;

#[no_mangle]
extern "win64" fn efi_main(handle: Handle, table: SystemTable) -> usize {
    console::text::output::set_stdout(table.console_out());
    console::text::output::clear();

    println!("Hello World");

    let mut buf = [0u8; 6000];
    let mem_map_iter = match table.boot().get_memory_map(&mut buf) {
        Ok((_key, mem_map_iter)) => mem_map_iter,
        Err(size) => {
            println!("required memory map size : {}", size);
            loop {}
        }
    };

    let sfs = table.boot().locate_protocol::<SimpleFileSystemProtocol>();
    let mut file = sfs
        .open_volume()
        .open("mem_map.csv", OpenMode::CreateReadWrite)
        .unwrap();

    // CSV header
    write!(
        file,
        "type, physical_start, virtual_start, num_pages, attrs\n"
    )
    .unwrap();

    for desc in mem_map_iter {
        write!(
            file,
            "{}, {:x}, {:x}, {}, {:x}\n",
            desc.typ, desc.physical_start, desc.virtual_start, desc.number_of_pages, desc.attribute
        )
        .unwrap();
    }

    file.flush();

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop {}
}
