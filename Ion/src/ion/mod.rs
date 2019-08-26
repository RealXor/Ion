use std::fmt::Error;
use std::sync::Mutex;

use crate::utils;
use crate::vmt::*;

mod sdk;
mod cheats;

lazy_static! {
    pub static ref HOOKS: Mutex<Vec<VMT>> = Mutex::new(vec![]);
    pub static ref INTERFACES: Mutex<sdk::interfaces::Interfaces> = Mutex::new(sdk::interfaces::Interfaces::default());
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

        let glow_object_manager: *const sdk::glow::CGlowObjectManager = *(res as *mut *mut usize) as _;

        /* yikes */
        let client_mode = **(((*((*(client as *mut *mut usize)).offset(10))) + 5) as *mut *mut *mut usize);

        let mut interfaces = INTERFACES.lock().unwrap();

        interfaces.client = sdk::client::CClient::from_raw(client);
        interfaces.client_mode = client_mode;
        interfaces.engine = sdk::engine::CEngine::from_raw(engine);
        interfaces.vgui_panel = sdk::panel::CPanel::from_raw(vgui_panel);
        interfaces.vgui_surface = sdk::surface::CSurface::from_raw(vgui_surface);
        interfaces.entity_list = sdk::entitylist::CEntityList::from_raw(entity_list);
        interfaces.debug_overlay = sdk::debugoverlay::CDebugOverlay::from_raw(debug_overlay);
        interfaces.glow_object_manager = glow_object_manager;

        println!("{:?}", interfaces);

        sdk::hook::hook();
        sdk::netvar::initialize();

        println!("Initialized");

        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }
}