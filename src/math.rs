use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn normalize(self) -> Vec3 {
        let len = self.len();
        self / len
    }

    pub fn len(&self) -> f32 {
        self.len_squre().sqrt()
    }

    pub fn len_squre(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

pub struct Ray3 {
    e: Vec3,
    d: Vec3,
}

impl Ray3 {
    pub fn new(e: Vec3, d: Vec3) -> Ray3 {
        assert!(1.0 - d.len_squre() < 0.00001);
        Ray3 { e, d }
    }

    pub fn hit_sphere(&self, sphere: &Sphere) -> Vec<f32> {
        let delta = self.e - sphere.c;
        let a = self.d * self.d;
        let b = 2.0 * (self.d * delta);
        let c = delta * delta - sphere.r * sphere.r;
        solve_quadratic_equations(a, b, c)
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.e + t * self.d
    }
}

pub fn solve_quadratic_equations(a: f32, b: f32, c: f32) -> Vec<f32> {
    let mut solve = Vec::new();

    let d = b * b - 4.0 * a * c;

    if d == 0.0 {
        solve.push(-b / (2.0 * a));
    } else if d > 0.0 {
        let d_sqrt = d.sqrt();
        solve.push((-b + d_sqrt) / (2.0 * a));
        solve.push((-b - d_sqrt) / (2.0 * a));
    }

    solve
}

pub struct Sphere {
    c: Vec3,
    r: f32,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32) -> Sphere {
        Sphere { c, r }
    }
}