#![no_std]
#![no_main]
#![feature(once_cell)]

mod uefi;
mod console;

use self::uefi::{proto::fs::SimpleFileSystemProtocol, Handle, SystemTable};

#[no_mangle]
extern "win64" fn efi_main(handle: Handle, table: SystemTable) -> usize {
    console::text::output::set_stdout(table.console_out());

    println!("Hello World");

    // write to file
    let sfs = table.boot().locate_protocol::<SimpleFileSystemProtocol>();

    if let Some(file) = sfs
        .open_volume()
        .open("test", 0x8000000000000000 | 0x01 | 0x02)
    {
        let status = file.write("hoge");
        println!("result of file write : {}", status);
        file.flush();
        println!("Success");
    } else {
        println!("Failed");
    }

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{:?}", info);
    loop {}
}
