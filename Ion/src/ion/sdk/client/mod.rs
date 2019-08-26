use std::mem::transmute;

use crate::ion::sdk::definitions;
use crate::utils;

#[derive(Debug)]
pub struct CClient {
    pub base: *mut usize,
}

type GetClientClassesFn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> *const definitions::clientclass::ClientClass;

impl CClient {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn get_all_classes(&self) -> *const definitions::clientclass::ClientClass {
        unsafe {
            transmute::<_, GetClientClassesFn>(utils::native::get_virtual_function(self.base, 8))(self.base)
        }
    }
}

impl Default for CClient {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}