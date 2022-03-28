#![no_std]
#![no_main]

use core::{ffi::c_void, ptr::NonNull};

#[no_mangle]
extern "win64" fn efi_main(handle: Handle, table: SystemTable) -> usize {
    // "Hello"
    let text = [0x0048, 0x0065, 0x006C, 0x006C, 0x006F, 0x0000];
    table.print(&text);

    // write to file
    let guid = Guid {
        data1: 0x0964e5b22,
        data2: 0x6459,
        data3: 0x11d2,
        data4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    };
    unsafe {
        let mut sfs: *mut c_void = core::ptr::null_mut();
        let status = ((*table.boot).locate_protocol)(&guid, core::ptr::null(), &mut sfs);
        let status_text = [0x0030 + status as u16, 0x0000];
        table.print(&status_text);

        let sfs = sfs.cast::<SimpleFileSystemProtocol>();
        if let Some(file) = (*sfs)
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
    }

    // "Hello42"
    let text = [
        0x0048, 0x0065, 0x006C, 0x006C, 0x006F, 0x0034, 0x0032, 0x0000,
    ];
    table.print(&text);

    loop {}
}

#[repr(transparent)]
struct Handle(NonNull<c_void>);

#[repr(C)]
struct SystemTable {
    header: [u8; 24],
    /// Null-terminated string representing the firmware's vendor.
    fw_vendor: usize,
    /// Revision of the UEFI specification the firmware conforms to.
    fw_revision: u32,
    console_in_handle: Handle,
    console_in: usize,
    console_out_handle: Handle,
    console_out: *const SimpleTextOutputProtocol,
    console_err_handle: Handle,
    console_err: usize,
    /// Runtime services table.
    runtime: usize,
    /// Boot services table.
    boot: *const BootServices,
    /// Number of entries in the configuration table.
    nr_cfg: usize,
    /// Pointer to beginning of the array.
    cfg_table: usize,
}

impl SystemTable {
    pub fn print(&self, text: &[u16]) {
        unsafe {
            ((*self.console_out).output_string)(self.console_out, text.as_ptr());
        }
    }
}

#[repr(C)]
struct BootServices {
    header: [u8; 24],

    // task priority services
    raise_tpl: usize,
    restore_tpl: usize,

    // memory services
    allocate_pages: usize,
    free_pages: usize,
    get_memory_map: usize,
    allocate_pool: usize,
    free_pool: usize,

    // event & timer
    create_event: usize,
    set_timer: usize,
    wait_for_event: usize,
    signal_event: usize,
    close_event: usize,
    check_event: usize,

    // protocol handler services
    install_protocol_interface: usize,
    rainstall_protocol_interface: usize,
    uninstall_protocol_interface: usize,
    handle_protocol: usize,
    reserved: *const c_void,
    register_protocol_notify: usize,
    locate_handle: usize,
    locate_device_path: usize,
    install_configuration_table: usize,

    // image services
    load_image: usize,
    start_image: usize,
    exit: usize,
    unload_image: usize,
    exit_boot_services: usize,

    // miscellaneous services
    get_next_monotonic_count: usize,
    stall: usize,
    set_watchdog_timer: usize,

    // driver support services
    connect_controller: usize,
    disconnect_controller: usize,

    // open and close protocol services
    open_protocol: usize,
    close_protocol: usize,
    open_protocol_information: usize,

    // library services
    protocols_per_handle: usize,
    locate_handle_buffer: usize,
    locate_protocol: unsafe extern "win64" fn(
        protocol: &Guid,
        registration: *const c_void,
        interface: &mut *mut c_void,
    ) -> usize,
    install_multiple_protocol_interfaces: usize,
    uninstall_multiple_protocol_interfaces: usize,

    // 32-bit CRC services
    calculate_crc_32: usize,

    // miscellaneous services
    copy_mem: usize,
    set_mem: usize,
    create_event_ex: usize,
}

#[repr(C)]
struct SimpleTextOutputProtocol {
    reset: usize,
    output_string: unsafe extern "win64" fn(
        this: *const SimpleTextOutputProtocol,
        string: *const u16,
    ) -> usize,
    test_string: usize,
    query_mode: usize,
    set_mode: usize,
    set_attribute: usize,
    clear_screen: usize,
    set_cursor_position: usize,
    enable_cursor: usize,
    mode: usize,
}

#[repr(C)]
struct SimpleFileSystemProtocol {
    revision: u64,
    open_volume: unsafe extern "win64" fn(
        this: &SimpleFileSystemProtocol,
        root: &mut *mut FileProtocol,
    ) -> usize,
}

impl SimpleFileSystemProtocol {
    pub fn open_volume(&self) -> &'static FileProtocol {
        let mut root: *mut FileProtocol = core::ptr::null_mut();
        unsafe {
            (self.open_volume)(self, &mut root);
            &*root
        }
    }
}

#[repr(C)]
struct FileProtocol {
    revision: u64,
    open: unsafe extern "win64" fn(
        this: &FileProtocol,
        new_handle: &mut *mut FileProtocol,
        file_name: *const u16,
        open_mode: u64,
        attributes: u64,
    ) -> usize,
    close: usize,
    delete: usize,
    read: usize,
    write: unsafe extern "win64" fn(
        this: &FileProtocol,
        buffer_size: &mut usize,
        buffer: *const c_void,
    ) -> usize,
    get_position: usize,
    set_position: usize,
    get_info: usize,
    set_info: usize,
    flush: unsafe extern "win64" fn(this: &FileProtocol) -> usize,
    open_ex: usize,
    read_ex: usize,
    write_ex: usize,
    flush_ex: usize,
}

impl FileProtocol {
    pub fn open(&self, file_name: &str, mode: u64) -> Option<&'static FileProtocol> {
        let mut new_handle: *mut FileProtocol = core::ptr::null_mut();

        let mut file_name_c16 = [0u16; 32];
        assert!(file_name.len() < 32);
        for (i, c) in file_name.encode_utf16().enumerate() {
            file_name_c16[i] = c;
        }

        unsafe {
            (self.open)(self, &mut new_handle, file_name_c16.as_ptr(), mode, 0);
            if new_handle.is_null() {
                return None;
            } else {
                return Some(&*new_handle);
            }
        }
    }

    pub fn write(&self, text: &str) -> usize {
        let mut buf_size = text.len();
        let buf = text.as_ptr().cast::<c_void>();

        unsafe { (self.write)(self, &mut buf_size, buf) }
    }

    pub fn flush(&self) -> usize {
        unsafe { (self.flush)(self) }
    }
}

#[repr(C)]
struct Guid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
