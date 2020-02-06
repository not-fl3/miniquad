extern "C" {
    pub fn fs_load_file(ptr: *const i8, len: u32) -> u32;
    pub fn fs_get_buffer_size(file_id: u32) -> i32;
    pub fn fs_take_buffer(file_id: u32, ptr: *mut u8, max_size: u32);
}
