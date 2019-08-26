
use winapi::{
    shared::minwindef::{HMODULE},
    ctypes::{c_void, c_char, c_int},
};

use crate::utils;
use crate::ion::sdk;

pub fn capture_interface(module: HMODULE, interface: *const u8) -> *const c_void {
    unsafe {
        let fn_addr = utils::native::get_proc_address(module, b"CreateInterface\0".as_ptr());

        let fn_capture_interface = std::mem::transmute::<*const c_void, extern "system" fn(*const c_char, *const c_int) -> *const c_void>(fn_addr);

        let interface_addr = fn_capture_interface(interface as _, std::ptr::null_mut());

        if !interface_addr.is_null() {
            println!("[capture_interface] captured {} at 0x{:X}", std::ffi::CStr::from_ptr(interface as _).to_str().unwrap(), interface_addr as usize);

            return interface_addr;
        }
    }
    std::ptr::null_mut()
}

#[derive(Debug)]
pub struct Interfaces {
    pub client: sdk::client::c_client,
    pub client_mode: *mut usize,
    pub vgui_surface: sdk::surface::c_surface,
    pub vgui_panel: sdk::panel::c_panel,
    pub entity_list: sdk::entitylist::c_entity_list,
    pub engine: sdk::engine::c_engine,
    pub glow_object_manager: *const sdk::glow::glow_object_manager_t,
    pub debug_overlay: sdk::debugoverlay::c_debugoverlay,
}

impl Default for Interfaces {
    fn default() -> Self {
        Self {
            client: sdk::client::c_client::default(),
            client_mode: std::ptr::null_mut(),
            engine: sdk::engine::c_engine::default(),
            glow_object_manager: std::ptr::null_mut(),
            vgui_panel: sdk::panel::c_panel::default(),
            entity_list: sdk::entitylist::c_entity_list::default(),
            vgui_surface: sdk::surface::c_surface::default(),
            debug_overlay: sdk::debugoverlay::c_debugoverlay::default(),
        }
    }
}

unsafe impl Send for Interfaces {}