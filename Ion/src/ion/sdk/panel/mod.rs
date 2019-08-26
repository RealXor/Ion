use std::os::raw::c_char;

use crate::utils;

#[derive(Debug)]
pub struct CPanel {
    pub base: *mut usize,
}


impl CPanel {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn get_panel_name(&self, panel: u32) -> *const c_char {
        type GetNameFn = unsafe extern "thiscall" fn(*const usize, u32) -> *const c_char;
        let vfunc = unsafe { std::mem::transmute::<_, GetNameFn>(utils::native::get_virtual_function(self.base, 36)) };

        unsafe {
            vfunc(self.base, panel)
        }
    }
}

impl Default for CPanel {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}