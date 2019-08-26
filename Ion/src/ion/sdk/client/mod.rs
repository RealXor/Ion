use std::mem::transmute;
use crate::utils;
use crate::ion::sdk::definitions;

#[derive(Debug)]
pub struct c_client {
    pub base: *mut usize,
}

type get_all_classes_fn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> *const definitions::clientclass::ClientClass;

impl c_client {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn get_all_classes(&self) -> *const definitions::clientclass::ClientClass {
        unsafe {
            transmute::<_, get_all_classes_fn>(utils::native::get_virtual_function(self.base,8))(self.base)
        }
    }
}

impl Default for c_client {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}