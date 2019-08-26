use std::mem::transmute;

use crate::utils;

type GetScreenSizeFn = unsafe extern "thiscall" fn(thisptr: *mut usize, width: &mut i32, height: &mut i32);
type GetLocalPlayerFn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> i32;

type ExecuteCmdFn = unsafe extern "thiscall" fn(thisptr: *mut usize, cmd: *const u8);

type ReturnsBoolFn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> bool;


#[derive(Debug)]
pub struct CEngine {
    base: *mut usize,
}

impl CEngine {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn get_screen_size(&self, width: &mut i32, height: &mut i32) {
        unsafe { transmute::<_, GetScreenSizeFn>(utils::native::get_virtual_function(self.base, 5))(self.base, width, height); }
    }

    pub fn is_ingame(&self) -> bool {
        unsafe { transmute::<_, ReturnsBoolFn>(utils::native::get_virtual_function(self.base, 26))(self.base) }
    }

    pub fn is_connected(&self) -> bool {
        unsafe { transmute::<_, ReturnsBoolFn>(utils::native::get_virtual_function(self.base, 27))(self.base) }
    }

    pub fn get_local_player(&self) -> i32 {
        unsafe { transmute::<_, GetLocalPlayerFn>(utils::native::get_virtual_function(self.base, 12))(self.base) }
    }

    pub fn execute_client_cmd(&self, cmd: *const u8) {
        unsafe { transmute::<_, ExecuteCmdFn>(utils::native::get_virtual_function(self.base, 108))(self.base, cmd); }
    }
}

impl Default for CEngine {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}