#![no_std]
#![no_main]

mod uefi;

use self::uefi::{proto::fs::SimpleFileSystemProtocol, Handle, SystemTable};

#[no_mangle]
extern "win64" fn efi_main(handle: Handle, table: SystemTable) -> usize {
    // "Hello"
    let text = [0x0048, 0x0065, 0x006C, 0x006C, 0x006F, 0x0000];
    table.print(&text);

    // write to file
    let sfs = table.boot().locate_protocol::<SimpleFileSystemProtocol>();

    if let Some(file) = sfs
        .open_volume()
        .open("test", 0x8000000000000000 | 0x01 | 0x02)
    {
        let status = file.write("hoge");
        let status_text = [0x0030 + status as u16, 0x0000];
        table.print(&status_text);

        let status = file.flush();
        let status_text = [0x0030 + status as u16, 0x0000];
        table.print(&status_text);
    } else {
        table.print(&text);
    }

    // "Hello42"
    let text = [
        0x0048, 0x0065, 0x006C, 0x006C, 0x006F, 0x0034, 0x0032, 0x0000,
    ];
    table.print(&text);

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
