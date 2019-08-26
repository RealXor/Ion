
use crate::utils::math;
use crate::ion::sdk::netvar;
use std::os::raw::c_char;
use std::ffi::{CString, CStr};
use crate::utils::math::vec::{Vec3, Matrix};
use std::intrinsics::transmute;
use crate::utils::native::get_virtual_function;

#[derive(Copy, Clone)]
pub struct c_entity {
    base: *mut usize,
}

type is_player_fn = unsafe extern "thiscall" fn(thisptr: *mut usize) -> bool;
type setup_bones_fn = unsafe extern "thiscall" fn(thisptr: *mut usize, out: &mut [Matrix], max_bones: i32, mask: i32, time: f32) -> bool;

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

    pub fn is_player(&self) -> bool {
        unsafe {
            transmute::<_, is_player_fn>(get_virtual_function(self.base, 155))(self.base)
        }
    }

    pub fn get_bone_matrix(&self) -> usize {
        self.get_value(netvar::get_offset("DT_BaseAnimating", "m_nForceBone") + 0x1C)
    }

    pub fn get_bone_pos(&self, bone: i32) -> Vec3 {
        let matrix = self.get_bone_matrix();
        self.get_value((matrix + bone as usize) * 0x30)
    }

    /// need to fix this
    pub fn setup_bones(&self, max_bones: i32, mask: i32, time: f32) -> Option<&[Matrix]> {
        None
    }
}