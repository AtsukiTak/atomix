pub mod text;
pub mod fs;
pub mod file;

#[repr(C)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub trait Protocol {
    const GUID: Guid;
}
