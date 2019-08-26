use winapi::{
    shared::minwindef::HMODULE,
};
use winapi::ctypes::{c_char, c_int, c_void};

use crate::ion::sdk;
use crate::utils;

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
    pub client: sdk::client::CClient,
    pub client_mode: *mut usize,
    pub vgui_surface: sdk::surface::CSurface,
    pub vgui_panel: sdk::panel::CPanel,
    pub entity_list: sdk::entitylist::CEntityList,
    pub engine: sdk::engine::CEngine,
    pub glow_object_manager: *const sdk::glow::CGlowObjectManager,
    pub debug_overlay: sdk::debugoverlay::CDebugOverlay,
}

impl Default for Interfaces {
    fn default() -> Self {
        Self {
            client: sdk::client::CClient::default(),
            client_mode: std::ptr::null_mut(),
            engine: sdk::engine::CEngine::default(),
            glow_object_manager: std::ptr::null_mut(),
            vgui_panel: sdk::panel::CPanel::default(),
            entity_list: sdk::entitylist::CEntityList::default(),
            vgui_surface: sdk::surface::CSurface::default(),
            debug_overlay: sdk::debugoverlay::CDebugOverlay::default(),
        }
    }
}

unsafe impl Send for Interfaces {}