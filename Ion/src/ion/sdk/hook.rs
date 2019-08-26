use std::os::raw::{c_float, c_void};

use crate::ion::*;
/// NOTE:
///     The use of "system" or "fastcall" is NOT arbitrary, in fact the reason of use for
///     fastcall in example the FSN hook, is because we might need to access the registers
///     provided


use crate::vmt::*;

type CreateMoveFn = unsafe extern "fastcall" fn(ecx: *const c_void, edx: *const c_void, _sampleframetime: c_float, *const sdk::definitions::cusercmd::CUserCMD) -> bool;
type FrameStageNotifyFn = unsafe extern "fastcall" fn(ecx: *const c_void, edx: *const c_void, stage: i32);
type PaintTraverseFn = unsafe extern "fastcall" fn(exc: *const c_void, edx: *const c_void, panel: u32, force_repaint: bool, allow_force: bool);

pub fn hook() {
    let add_vmt = |vmt: VMT| { HOOKS.lock().unwrap().push(vmt); };

    let mut client_mode_vmt = VMT::new(INTERFACES.lock().unwrap().client_mode);
    let mut client_vmt = VMT::new(INTERFACES.lock().unwrap().client.base);
    let mut panel_vmt = VMT::new(INTERFACES.lock().unwrap().vgui_panel.base);

    client_mode_vmt.hook(24, create_move as _);
    client_vmt.hook(37, fsn as _);
    panel_vmt.hook(41, paint_traverse as _);

    add_vmt(client_mode_vmt);
    add_vmt(client_vmt);
    add_vmt(panel_vmt);
}

unsafe extern "fastcall" fn create_move(ecx: *const c_void, edx: *const c_void, _sampleframetime: c_float, cmd: *const sdk::definitions::cusercmd::CUserCMD) -> bool {
    if cmd.is_null() || cmd.read().command_number == 0 || !INTERFACES.lock().unwrap().engine.is_ingame()
        || !INTERFACES.lock().unwrap().engine.is_connected() {
        return std::mem::transmute::<_, CreateMoveFn>(HOOKS.lock().unwrap()[0].get_original(24))(ecx, edx, _sampleframetime, cmd);
    }

    false
}

unsafe extern "fastcall" fn fsn(exc: *const c_void, edx: *const c_void, stage: i32) {
    std::mem::transmute::<_, FrameStageNotifyFn>(HOOKS.lock().unwrap()[1].get_original(37))(exc, edx, stage);
}

unsafe extern "fastcall" fn paint_traverse(exc: *const c_void, edx: *const c_void, panel: u32, force_repaint: bool, allow_force: bool) {
    use std::ffi::CStr;

    let original = std::mem::transmute::<_, PaintTraverseFn>(HOOKS.lock().unwrap()[2].get_original(41));

    // Will be used for drawing
    static mut PANEL_ID: u32 = 0;
    // Will be implemented later for no scope
    static mut PANEL_HUD_ID: u32 = 0;

    if PANEL_HUD_ID == 0 {
        let panel_name = INTERFACES.lock().unwrap().vgui_panel.get_panel_name(panel);

        let c_str = CStr::from_ptr(panel_name);
        let string = c_str.to_str().unwrap();

        if string.contains("HudZoom") {
            PANEL_HUD_ID = panel;
        }
    }

    if PANEL_ID == 0 {
        let panel_name = INTERFACES.lock().unwrap().vgui_panel.get_panel_name(panel);

        let c_str = CStr::from_ptr(panel_name);
        let string = c_str.to_str().unwrap();

        if string.contains("MatSystemTopPanel") {
            PANEL_ID = panel;
        }
    }

    original(exc, edx, panel, force_repaint, allow_force);

    // Top panel, so that we can draw :)
    if PANEL_ID == panel {
        // DRAW
        cheats::visuals::draw_visuals();
    }
}
