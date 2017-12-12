pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
}

pub struct Ray3 {
    p: Vec3,
    d: Vec3,
}

impl Ray3 {
    pub fn new(p: Vec3, d: Vec3) -> Ray3 {
        Ray3 { p, d }
    }
}