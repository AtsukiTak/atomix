use core::{ffi::c_void, fmt};

#[repr(C)]
pub struct FileProtocol {
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
    pub fn open(&self, file_name: &str, mode: OpenMode) -> Option<&'static FileProtocol> {
        let mut new_handle: *mut FileProtocol = core::ptr::null_mut();

        let mut file_name_c16 = [0u16; 32];
        assert!(file_name.len() < 32);
        for (i, c) in file_name.encode_utf16().enumerate() {
            file_name_c16[i] = c;
        }

        unsafe {
            (self.open)(
                self,
                &mut new_handle,
                file_name_c16.as_ptr(),
                mode as u64,
                0,
            );
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

impl fmt::Write for FileProtocol {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

impl fmt::Write for &FileProtocol {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

#[repr(u64)]
pub enum OpenMode {
    Read = 0x01,
    ReadWrite = 0x01 | 0x02,
    CreateReadWrite = (1 << 63) | 0x01 | 0x02,
}
