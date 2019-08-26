use std::mem::*;

use crate::utils::native::get_virtual_function;

#[derive(Debug)]
pub struct CEntityList {
    base: *mut usize,
}

type GetEntityByIdFN = unsafe extern "thiscall" fn(thisptr: *mut usize, id: i32) -> *mut usize;
type GetHighestEntityIndexFn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> i32;

impl CEntityList {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn get_entity_by_id(&self, yoehhwtfwhyisthisnotworking: i32) -> *mut usize {
        unsafe {
            let func = transmute::<_, GetEntityByIdFN>(get_virtual_function(self.base, 3));
            func(self.base, yoehhwtfwhyisthisnotworking)
        }
    }

    pub fn get_highest_ent_idx(&self) -> i32 {
        unsafe {
            transmute::<_, GetHighestEntityIndexFn>(get_virtual_function(self.base, 6))(self.base)
        }
    }
}

impl Default for CEntityList {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}