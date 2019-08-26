use std::mem::transmute;

use crate::utils::math::vec::Vec3;
use crate::utils::native::get_virtual_function;

type WorldToScreenFn = unsafe extern "thiscall" fn(thisptr: *mut usize, input: *const Vec3, out: *mut Vec3) -> i32;

#[derive(Debug)]
pub struct CDebugOverlay {
    base: *mut usize,
}

impl CDebugOverlay {

    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn world_to_screen(&self, position: &Vec3) -> Option<Vec3> {
        let mut return_vec = unsafe {std::mem::zeroed()};
        let return_code = unsafe {
            transmute::<_, WorldToScreenFn>(get_virtual_function(self.base, 13))(self.base, position as *const _, &mut return_vec as *mut _)
        };

        if return_code == 1 {
            return None;
        }

        Some(return_vec)
    }

}

impl Default for CDebugOverlay {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}

