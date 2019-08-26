use crate::vmt::*;
use crate::utils;

use std::fmt::Error;
use std::sync::Mutex;

mod sdk;
mod cheats;

use winapi::shared::minwindef::DWORD;

use std::os::raw::c_float;

use std::sync::RwLock;
use winapi::ctypes::c_void;
use std::process::exit;

lazy_static! {
    pub static ref hooks: Mutex<Vec<VMT>> = Mutex::new(vec![]);
    pub static ref interfaces: Mutex<sdk::interfaces::Interfaces> = Mutex::new(sdk::interfaces::Interfaces::default());
}

pub fn start() {

    let client_panorama = utils::native::get_module_handle(b"client_panorama.dll\0".as_ptr());
    let vgui_factory = utils::native::get_module_handle(b"vguimatsurface.dll\0".as_ptr());
    let vgui2_factory = utils::native::get_module_handle(b"vgui2.dll\0".as_ptr());
    let engine_factory = utils::native::get_module_handle(b"engine.dll\0".as_ptr());

    unsafe {
        let res = utils::sig::pattern_scan(client_panorama, "0F 11 05 ?? ?? ?? ?? 83 C8 01 C7 05 ?? ?? ?? ?? 00 00 00 00") as usize + 3;

        let client = sdk::interfaces::capture_interface(client_panorama, b"VClient018\0".as_ptr()) as *mut usize;
        let engine = sdk::interfaces::capture_interface(engine_factory, b"VEngineClient014\0".as_ptr()) as *mut usize;
        let entity_list = sdk::interfaces::capture_interface(client_panorama, b"VClientEntityList003\0".as_ptr()) as *mut usize;
        let vgui_panel = sdk::interfaces::capture_interface(vgui2_factory, b"VGUI_Panel009\0".as_ptr()) as *mut usize;
        let vgui_surface = sdk::interfaces::capture_interface(vgui_factory, b"VGUI_Surface031\0".as_ptr()) as *mut usize;
        let debug_overlay = sdk::interfaces::capture_interface(engine_factory, b"VDebugOverlay004\0".as_ptr()) as *mut usize;

        let glow_object_manager: *const sdk::glow::glow_object_manager_t = *(res as *mut *mut usize) as _;

        /* yikes */
        let client_mode = **(((*((*(client as *mut *mut usize)).offset(10))) + 5) as *mut *mut *mut usize);

        interfaces.lock().unwrap().client = sdk::client::c_client::from_raw(client);
        interfaces.lock().unwrap().client_mode = client_mode;
        interfaces.lock().unwrap().engine = sdk::engine::c_engine::from_raw(engine);
        interfaces.lock().unwrap().vgui_panel = sdk::panel::c_panel::from_raw(vgui_panel);
        interfaces.lock().unwrap().vgui_surface = sdk::surface::c_surface::from_raw(vgui_surface);
        interfaces.lock().unwrap().entity_list = sdk::entitylist::c_entity_list::from_raw(entity_list);
        interfaces.lock().unwrap().debug_overlay = sdk::debugoverlay::c_debugoverlay::from_raw(debug_overlay);
        interfaces.lock().unwrap().glow_object_manager = glow_object_manager;

        println!("{:?}", interfaces.lock().unwrap());

        sdk::hook::hook();
        sdk::netvar::initialize();

        println!("Initialized");

        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }
}