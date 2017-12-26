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
            let filename = CString::new(path).unwrap();
            stbi_write_png(filename.as_ptr(),
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
        let p = Vec3::new(x, y, 0.0);
        Ray3::new(self.p, (p - self.p).normalize())
    }
}

pub struct Light {
    p: Vec3,
    i: Vec3,
}

fn main() {
    let background_color = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut ib = ImageBuffer::new(1920, 1080);
    let camera = Camera::new(Vec3::new(960.0, 540.0, 2000.0));
    let ambient_color = Vec3::new(0.2, 0.2, 0.2);
    let mut spheres = vec![Sphere::new(Vec3::new(960.0, 540.0, -100.0), 100.0),
                           Sphere::new(Vec3::new(1100.0, 500.0, 0.0), 50.0),
                           Sphere::new(Vec3::new(1300.0, 640.0, -90.0), 90.0),
                           Sphere::new(Vec3::new(760.0, 340.0, -120.0), 80.0)];

    let lights = vec![
        Light {
            p: Vec3::new(990.0, 330.0, 80.0),
            i: Vec3::new(0.7, 0.2, 0.2),
        },
        Light {
            p: Vec3::new(300.0, 330.0, 80.0),
            i: Vec3::new(0.2, 0.7, 0.2),
        },
        Light {
            p: Vec3::new(500.0, 830.0, 80.0),
            i: Vec3::new(0.2, 0.2, 0.7),
        },
    ];

    spheres.extend(lights.iter().map(|light| Sphere::new(light.p, 6.0)));

    for y in 0..ib.height {
        for x in 0..ib.width {
            #[derive(Debug)]
            struct Hit<'a> {
                t: f32,
                sphere: &'a Sphere,
            }

            let mut color = background_color;
            let camera_ray = camera.calc_camera_ray(&ib, x, y);
            let mut camera_ray_hits = spheres
                .iter()
                .flat_map(|sphere| {
                              camera_ray
                                  .hit_sphere(sphere)
                                  .into_iter()
                                  .map(|t| Hit { t, sphere })
                                  .collect::<Vec<_>>()
                          })
                .collect::<Vec<_>>();
            camera_ray_hits.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

            if camera_ray_hits.len() > 0 {
                let sphere = camera_ray_hits[0].sphere;
                let t = camera_ray_hits[0].t;
                let p = camera_ray.at(t);
                let n = sphere.normal_at(p);
                let v = -camera_ray.d;

                let mut light_color = Vec3::new(0.0, 0.0, 0.0);

                for light in lights.iter() {
                    if sphere.c == light.p {
                        light_color = light.i + light.i;
                        break;
                    } else {
                        let shadow_ray_d = (light.p - p).normalize();
                        let shadow_ray = Ray3::new(p + 0.1 * shadow_ray_d, shadow_ray_d);
                        let mut is_in_shadow = false;
                        for sphere in spheres.iter() {
                            if lights
                                   .iter()
                                   .find(|light| light.p == sphere.c)
                                   .is_some() {
                                continue;
                            }

                            let hits = shadow_ray.hit_sphere(sphere);
                            if hits.len() > 0 {
                                is_in_shadow = true;
                                break;
                            }
                        }

                        if !is_in_shadow {
                            let l = (light.p - p).normalize();
                            let h = (l + v).normalize();
                            light_color = light_color +
                                          (light.i * (n * l).max(0.0) +
                                           light.i * (n * h).max(0.0).powf(100.0));
                        }
                    }
                }

                color = ambient_color + light_color;
            }

            color = color.clamp(1.0);
            ib.set_pixel(x,
                         y,
                         rgba((color.x * 255.0).round() as u8,
                              (color.y * 255.0).round() as u8,
                              (color.z * 255.0).round() as u8,
                              255));
        }
    }

    ib.save("output.png");
}
