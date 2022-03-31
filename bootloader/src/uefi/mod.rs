pub mod proto;

use self::proto::{
    text::SimpleTextOutputProtocol,
    {Guid, Protocol},
};
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
    get_memory_map: unsafe extern "win64" fn(
        memory_map_size: &mut usize,
        memory_map: *mut MemoryDescriptor,
        map_key: &mut MapKey,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> usize,
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
    pub fn get_memory_map<'buf>(
        &self,
        buf: &'buf mut [u8],
    ) -> Result<(MapKey, MemoryMapIter<'buf>), usize> {
        let mut mem_map_size = buf.len();
        let mem_map = buf.as_mut_ptr().cast();
        let mut map_key = MapKey(0);
        let mut desc_size = 0usize;
        let mut desc_ver = 0u32;

        let status = unsafe {
            (self.get_memory_map)(
                &mut mem_map_size,
                mem_map,
                &mut map_key,
                &mut desc_size,
                &mut desc_ver,
            )
        };

        if status != 0 {
            return Err(mem_map_size);
        } else {
            let mem_map_iter = MemoryMapIter {
                buf,
                desc_size,
                index: 0,
                len: mem_map_size / desc_size,
            };
            Ok((map_key, mem_map_iter))
        }
    }

    pub fn locate_protocol<P: Protocol>(&self) -> &P {
        let mut interface: *mut c_void = core::ptr::null_mut();
        unsafe {
            let _status = (self.locate_protocol)(&P::GUID, core::ptr::null(), &mut interface);
            &*interface.cast::<P>()
        }
    }
}

#[repr(transparent)]
pub struct MapKey(usize);

#[repr(C)]
#[derive(Debug)]
pub struct MemoryDescriptor {
    typ: u32,
    physical_start: PhysicalAddress,
    virtual_start: VirtualAddress,
    number_of_pages: u64,
    attribute: u64,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct PhysicalAddress(u64);

#[repr(transparent)]
#[derive(Debug)]
pub struct VirtualAddress(u64);

pub struct MemoryMapIter<'buf> {
    buf: &'buf mut [u8],
    // size of memory descriptor in bytes
    desc_size: usize,
    index: usize,
    len: usize,
}

impl<'buf> Iterator for MemoryMapIter<'buf> {
    type Item = &'buf MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }

        let desc = unsafe {
            let ptr = self.buf.as_ptr() as usize + self.index * self.desc_size;
            &*(ptr as *const MemoryDescriptor)
        };

        self.index += 1;

        Some(desc)
    }
}
