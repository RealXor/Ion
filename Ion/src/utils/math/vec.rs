use std::ops;
use std::ops::Add;

use winapi::ctypes::c_float;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2 {
    pub yaw: c_float,
    pub pitch: c_float,
    unused: c_float,
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(mut self, rhs: Vec3) -> Self {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: Vec3) -> Self {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x, y, z,
        }
    }

    pub fn empty() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
