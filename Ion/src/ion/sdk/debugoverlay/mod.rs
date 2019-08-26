use std::mem::transmute;
use crate::utils::math::vec::{Vec3, Vec2};
use crate::utils::native::get_virtual_function;

type world_to_screen_fn = unsafe extern "thiscall" fn(thisptr: *mut usize, input: Vec3, out: *mut Vec2) -> i32;

#[derive(Debug)]
pub struct c_debugoverlay {
    base: *mut usize,
}

impl c_debugoverlay {

    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn world_to_screen(&self, position: Vec3) -> Option<Vec2> {
        let mut return_vec = unsafe {std::mem::zeroed()};
        let return_code = unsafe {
            transmute::<_, world_to_screen_fn>(get_virtual_function(self.base, 13))(self.base, position, &mut return_vec as *mut _)
        };

        if return_code != 0 {
            return None;
        }

        Some(return_vec)
    }

}

impl Default for c_debugoverlay {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}

