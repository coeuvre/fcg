extern crate fcg;

use std::os::raw::c_void;
use std::ffi::CString;
use fcg::stb_image_write::stbi_write_png;
use fcg::math::*;

pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    data: Vec<u32>,
}

impl ImageBuffer {
    pub fn new(width: u32, height: u32) -> ImageBuffer {
        ImageBuffer {
            width,
            height,
            data: vec![0; (width * height) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: u32) {
        *self.data
             .get_mut((y * self.width + x) as usize)
             .unwrap() = color;
    }

    pub fn save(&self, path: &str) {
        unsafe {
            stbi_write_png(CString::new(path).unwrap().as_ptr(),
                           self.width as i32,
                           self.height as i32,
                           4,
                           self.data.as_ptr() as *const c_void,
                           (4 * self.width) as i32);
        }
    }
}

pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (r as u32) + ((g as u32) << 8) + ((b as u32) << 16) + ((a as u32) << 24)
}

pub struct Camera {
    p: Vec3,
}

impl Camera {
    pub fn new(p: Vec3) -> Camera {
        Camera { p }
    }

    pub fn calc_camera_ray(&self, ib: &ImageBuffer, x_pixel: u32, y_pixel: u32) -> Ray3 {
        let x = x_pixel as f32;
        let y = (ib.height - y_pixel - 1) as f32;
        // let p = Vec3::new(x, y, 0.0);
        Ray3::new(Vec3::new(x, y, self.p.z), Vec3::new(0.0, 0.0, -1.0))
    }
}

fn main() {
    const BACKGROUND_COLOR: u32 = 0xFF000000;
    let mut ib = ImageBuffer::new(1920, 1080);
    let camera = Camera::new(Vec3::new(960.0, 540.0, 50.0));
    let light_p = Vec3::new(990.0, 590.0, 100.0);
    let ambient_color = Vec3::new(0.1, 0.1, 0.1);
    let spheres = vec![Sphere::new(Vec3::new(960.0, 540.0, -100.0), 100.0),
                       Sphere::new(Vec3::new(1300.0, 640.0, -90.0), 90.0),
                       Sphere::new(Vec3::new(760.0, 340.0, -120.0), 80.0)];

    for y in 0..ib.height {
        for x in 0..ib.width {
            struct Hit<'a> {
                t: f32,
                sphere: &'a Sphere,
            }

            let camera_ray = camera.calc_camera_ray(&ib, x, y);
            let mut hits = spheres
                .iter()
                .flat_map(|sphere| {
                              camera_ray
                                  .hit_sphere(sphere)
                                  .into_iter()
                                  .map(|t| Hit { t, sphere })
                                  .collect::<Vec<_>>()
                          })
                .collect::<Vec<_>>();
            hits.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
            if hits.len() > 0 {
                let hit = camera_ray.at(hits[0].t);
                let n = hits[0].sphere.normal_at(hit);
                let l = (light_p - hit).normalize();
                let v = -camera_ray.d;
                let h = (l + v).normalize();
                let i = Vec3::new(1.0, 1.0, 1.0);
                let color = (ambient_color + i * (n * l).max(0.0) +
                             i * (n * h).max(0.0).powf(100.0))
                        .clamp(1.0);
                ib.set_pixel(x,
                             y,
                             rgba((color.x * 255.0).round() as u8,
                                  (color.y * 255.0).round() as u8,
                                  (color.z * 255.0).round() as u8,
                                  255));
            } else {
                ib.set_pixel(x, y, BACKGROUND_COLOR);
            }
        }
    }

    ib.save("output.png");
}
