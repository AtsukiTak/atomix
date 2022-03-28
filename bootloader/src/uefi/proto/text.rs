#[repr(C)]
pub struct SimpleTextOutputProtocol {
    reset: usize,
    output_string:
        unsafe extern "win64" fn(this: &SimpleTextOutputProtocol, string: *const u16) -> usize,
    test_string: usize,
    query_mode: usize,
    set_mode: usize,
    set_attribute: usize,
    clear_screen: usize,
    set_cursor_position: usize,
    enable_cursor: usize,
    mode: usize,
}

impl SimpleTextOutputProtocol {
    pub fn output_string(&self, s: *const u16) -> usize {
        unsafe { (self.output_string)(self, s) }
    }
}
