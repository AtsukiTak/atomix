#![no_std]
#![no_main]

use core::fmt::Write;
use uefi::{
    table::{Boot, SystemTable},
    Handle, ResultExt as _, Status,
};

/// x64プラットフォームでは、UEFIアプリケーションの
/// 呼出規約は、"win64" (MS_ABI) と等しい。
/// ちなみにRISC-Vなど他のプラットフォームでは、
/// "C" (System-V ABI) と等しい。
/// また、rustにはこれをターゲットごとに自動で
/// 設定してくれる "efiapi" という規約もある(unstable)
#[no_mangle]
extern "win64" fn efi_main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    st.stdout().clear().unwrap_success();
    writeln!(st.stdout(), "Hello, world!").unwrap();

    let vendor = unsafe { uefi::CStr16::from_ptr::<'static>(st.firmware_vendor().as_ptr()) };
    writeln!(st.stdout(), "vendor : {}", vendor).unwrap();

    let firm_rev = st.firmware_revision();
    writeln!(st.stdout(), "firmware revision : {:?}", firm_rev).unwrap();

    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
