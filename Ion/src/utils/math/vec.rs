
use winapi::ctypes::c_float;
use std::ops::Add;

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
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Matrix {
    pub mat_val: [[f32; 4]; 3],
}

use std::ops;

impl ops::Add<Vec3> for Vec3 {
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

    fn sub(mut self, rhs: Vec3) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: Vec3) -> Self::Output {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
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
            x: 0 as f32, y: 0 as f32, z: 0 as f32,
        }
    }
}

impl Matrix {
    pub fn empty() -> Self {
        unsafe {
            Self {
                mat_val: std::mem::zeroed(),
            }
        }
    }
}