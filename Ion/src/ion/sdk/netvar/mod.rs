use crate::ion::*;
use crate::ion::sdk::definitions;
use crate::ion::sdk::definitions::recvprop::{c_recv_table, c_recv_prop, e_prop_type};
use std::ffi::CStr;

use winapi::{
    shared::minwindef::{DWORD},
    um::{
        memoryapi::{VirtualProtect},
        winnt::{PAGE_READWRITE},
    },
};
use std::collections::HashMap;

pub static mut TABLES: Vec<*mut c_recv_table> = Vec::new();

lazy_static! {
    pub static ref netvars: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

pub fn store_props(group_name: String, recv_table: *mut c_recv_table, child_offset: usize) {
    unsafe {
        for i in 0..(*recv_table).n_props as isize {
            let prop = (*recv_table).p_props.offset(i).read();
            let child = prop.data_table;

            if !child.is_null() && (*child).n_props > 0 {
                store_props(group_name.to_owned(), child, prop.offset as usize);
            }

            let var_name = CStr::from_ptr(prop.prop_name).to_str().unwrap().to_owned();

            let formatted = format!("{}->{}", group_name, var_name);

            if !netvars.lock().unwrap().contains_key(&var_name) && (
                prop.prop_type == e_prop_type::Int ||
                prop.prop_type == e_prop_type::Vec ||
                prop.prop_type == e_prop_type::VecXY ||
                prop.prop_type == e_prop_type::String ||
                prop.prop_type == e_prop_type::Float) {
                netvars.lock().unwrap().insert(formatted.to_owned(), prop.offset as usize + child_offset as usize);
            }
        }
    }
}

pub fn initialize() -> bool {
    unsafe {
        TABLES.clear();

        let mut client_class_ptr: *const definitions::clientclass::ClientClass  = interfaces.lock().unwrap().client.get_all_classes();

        if client_class_ptr.is_null() {
            return false;
        }

        while !client_class_ptr.is_null() {
            let recv_table = client_class_ptr.read().recv_table;
            TABLES.push(recv_table);

            let table_name = CStr::from_ptr(recv_table.read().table_name).to_str().unwrap().to_owned();
            store_props(table_name, recv_table, 0);

            client_class_ptr = client_class_ptr.read().next as *const definitions::clientclass::ClientClass;
        }
    }

    true
}

pub fn get_offset(table: &str, netvar: &str) -> usize {
    let formatted_key = format!("{}->{}", table, netvar);
    if netvars.lock().unwrap().contains_key(&formatted_key.to_owned()) {
        return *netvars.lock().unwrap().get(&formatted_key).unwrap();
    }
    0
}
