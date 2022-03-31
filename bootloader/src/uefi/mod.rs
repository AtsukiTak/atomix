pub mod proto;

use self::proto::{text::SimpleTextOutputProtocol, {Protocol, Guid}};
use core::{ffi::c_void, ptr::NonNull};

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Handle(NonNull<c_void>);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SystemTable {
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
    pub fn console_out(&self) -> &'static SimpleTextOutputProtocol {
        unsafe { &*self.console_out }
    }

    pub fn boot(&self) -> &BootServices {
        unsafe { &*self.boot }
    }
}

#[repr(C)]
pub struct BootServices {
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

impl BootServices {
    pub fn locate_protocol<P: Protocol>(&self) -> &P {
        let mut interface: *mut c_void = core::ptr::null_mut();
        unsafe {
            let _status = (self.locate_protocol)(&P::GUID, core::ptr::null(), &mut interface);
            &*interface.cast::<P>()
        }
    }
}
