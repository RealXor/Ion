use std::mem::transmute;

type get_screen_size_fn = unsafe extern "thiscall" fn(thisptr: *mut usize, width: &mut i32, height: &mut i32);
type get_local_player_fn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> i32;

type execute_cmd_fn = unsafe extern "thiscall" fn(thisptr: *mut usize, cmd: *const u8);

type returns_bool = unsafe extern "thiscall" fn(thisptr: *mut usize) -> bool;


#[derive(Debug)]
pub struct c_engine {
    base: *mut usize,
}

use crate::utils;

impl c_engine {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn get_screen_size(&self, width: &mut i32, height: &mut i32) {
        unsafe { transmute::<_, get_screen_size_fn>(utils::native::get_virtual_function(self.base, 5))(self.base, width, height); }
    }

    pub fn is_ingame(&self) -> bool {
        unsafe { transmute::<_, returns_bool>(utils::native::get_virtual_function(self.base, 26))(self.base) }
    }

    pub fn is_connected(&self) -> bool {
        unsafe { transmute::<_, returns_bool>(utils::native::get_virtual_function(self.base, 27))(self.base) }
    }

    pub fn get_local_player(&self) -> i32 {
        unsafe {
            transmute::<_, get_local_player_fn>(utils::native::get_virtual_function(self.base, 12))(self.base)
        }
    }

    pub fn execute_client_cmd(&self, cmd: *const u8) {
        unsafe {
            transmute::<_, execute_cmd_fn>(utils::native::get_virtual_function(self.base, 108))(self.base, cmd);
        }
    }
}

impl Default for c_engine {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}