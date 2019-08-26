use std::ffi::{CStr, CString};
use std::intrinsics::transmute;
use std::os::raw::c_char;

use crate::ion::sdk::netvar;
use crate::utils::math;
use crate::utils::math::vec::{Matrix, Vec3};
use crate::utils::native::get_virtual_function;

#[derive(Copy, Clone)]
pub struct c_entity {
    base: *mut usize,
}

type is_player_fn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> bool;
type setup_bones_fn = unsafe extern "thiscall" fn(thisptr: *mut usize, out: *mut Matrix, max_bones: i32, mask: i32, time: f32) -> bool;

/// Note:
///     offsets are hardcoded as of 22/8/19
///     I need to get to it, calm down
impl c_entity {

    pub fn get_value<T>(&self, offset: usize) -> T {
        unsafe {
           ((self.base as usize + offset) as *mut T).read()
        }
    }

    pub unsafe fn from_raw(base: *mut usize) -> Self {
        Self {
            base,
        }
    }

    pub fn get_health(&self) -> i32 {
        self.get_value(netvar::get_offset("DT_BasePlayer", "m_iHealth"))
    }

    pub fn get_armor(&self) -> i32 {
        self.get_value(netvar::get_offset("DT_BasePlayer", "m_ArmorValue"))
    }

    pub fn get_aim_punch(&self) -> math::vec::Vec3 {
        self.get_value(netvar::get_offset("DT_BasePlayer", "m_aimPunchAngle"))
    }

    pub fn is_scoped(&self) -> bool {
        self.get_value(netvar::get_offset("DT_BasePlayer", "m_bIsScoped"))
    }

    pub fn is_defusing(&self) -> bool {
        self.get_value(netvar::get_offset("DT_BasePlayer", "m_bIsDefusing"))
    }

    pub fn get_team_num(&self) -> i32 {
        self.get_value(netvar::get_offset("DT_BasePlayer", "m_iTeamNum"))
    }

    pub fn get_origin(&self) -> math::vec::Vec3 {
        self.get_value(netvar::get_offset("DT_BasePlayer", "m_vecOrigin"))
    }

    pub fn get_velocity(&self) -> math::vec::Vec3 {
        self.get_value(netvar::get_offset("DT_BasePlayer", "m_vecVelocity"))
    }

    pub fn get_name(&self) -> String {
        let name: [c_char; 260] = self.get_value(netvar::get_offset("DT_BasePlayer", "m_iName"));
        unsafe { CStr::from_ptr(name.as_ptr()).to_str().unwrap().to_owned() }
    }

    pub fn is_dormant(&self) -> bool {
        self.get_value(0xED)
    }

    pub fn is_player(&self) -> bool {
        unsafe {
            transmute::<_, is_player_fn>(get_virtual_function(self.base, 155))(self.base)
        }
    }

    pub fn get_life_state(&self) -> i32 {
        self.get_value(netvar::get_offset("DT_BasePlayer", "m_lifeState"))
    }

    pub fn is_alive(&self) -> bool {
        self.get_life_state() == 0
    }

    pub fn get_animating(&self) -> *mut usize {
        self.get_value(0x4)
    }

    pub fn get_bone_pos(&self, bone: i32) -> Vec3 {
        let ptr: *mut usize = self.get_value(0x26a8);

        let x = unsafe { *((ptr as usize + (bone as usize * 48 + 12)) as *mut f32) };
        let y = unsafe { *((ptr as usize + (bone as usize * 48 + 28)) as *mut f32) };
        let z = unsafe { *((ptr as usize + (bone as usize * 48 + 44)) as *mut f32) };

        Vec3::new(x, y, z)

    }
}