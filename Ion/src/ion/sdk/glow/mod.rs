use winapi::{
  ctypes::{c_char, c_int},
};

#[repr(C)]
pub struct CGlowObjectManager {
    padding_01: [c_char; 4], // 0
    pub total_glow_objects: c_int, //4
    padding_02: [c_char; 4], //8
    pub current_glow_objects: c_int, //12
}