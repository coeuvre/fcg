use std::os::raw::{c_char, c_int, c_void};

extern "C" {
    pub fn stbi_write_png(filename: *const c_char,
                          w: c_int,
                          h: c_int,
                          comp: c_int,
                          data: *const c_void,
                          stride_in_bytes: c_int)
                          -> c_int;
}