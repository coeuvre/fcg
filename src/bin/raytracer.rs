extern crate fcg;

use std::os::raw::c_void;
use std::ffi::CString;
use fcg::stb_image_write::stbi_write_png;

pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    data: Vec<u8>,
}

impl ImageBuffer {
    pub fn new(width: u32, height: u32) -> ImageBuffer {
        ImageBuffer {
            width,
            height,
            data: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        *self.data
             .get_mut((y * self.width * 4 + x * 4 + 0) as usize)
             .unwrap() = r;
        *self.data
             .get_mut((y * self.width * 4 + x * 4 + 1) as usize)
             .unwrap() = g;
        *self.data
             .get_mut((y * self.width * 4 + x * 4 + 2) as usize)
             .unwrap() = b;
        *self.data
             .get_mut((y * self.width * 4 + x * 4 + 3) as usize)
             .unwrap() = a;
    }

    pub fn save(&self, path: &str) {
        unsafe {
            stbi_write_png(CString::new(path).unwrap().as_ptr(),
                           self.width as i32,
                           self.height as i32,
                           4,
                           self.data.as_ptr() as *const c_void,
                           (self.width * 4) as i32);
        }
    }
}

fn main() {
    let ib = ImageBuffer::new(1280, 720);
    ib.save("output.png");
}
