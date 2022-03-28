use super::{file::FileProtocol, Guid, Protocol};

#[repr(C)]
pub struct SimpleFileSystemProtocol {
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

impl Protocol for SimpleFileSystemProtocol {
    const GUID: Guid = Guid {
        data1: 0x0964e5b22,
        data2: 0x6459,
        data3: 0x11d2,
        data4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    };
}
