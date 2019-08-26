use crate::utils::native;
use std::mem::*;
use crate::utils::native::get_virtual_function;
use crate::ion::sdk::definitions::entity;
use winapi::ctypes::c_void;

#[derive(Debug)]
pub struct c_entity_list {
    base: *mut usize,
}

type get_entity_byid = unsafe extern "thiscall" fn(thisptr: *mut usize, id: i32) -> *mut usize;
type get_highest_ent_index = unsafe extern "thiscall" fn(thisptr: *mut usize) -> i32;

impl c_entity_list {
    pub unsafe fn from_raw(addr: *mut usize) -> Self {
        Self {
            base: addr,
        }
    }

    pub fn get_entity_by_id(&self, yoehhwtfwhyisthisnotworking: i32) -> *mut usize {
        unsafe {
            let func = transmute::<_, get_entity_byid>(get_virtual_function(self.base, 3));
            func(self.base, yoehhwtfwhyisthisnotworking)
        }
    }

    pub fn get_highest_ent_idx(&self) -> i32 {
        unsafe {
            transmute::<_, get_highest_ent_index>(get_virtual_function(self.base, 6))(self.base)
        }
    }
}

impl Default for c_entity_list {
    fn default() -> Self {
        Self {
            base: std::ptr::null_mut(),
        }
    }
}