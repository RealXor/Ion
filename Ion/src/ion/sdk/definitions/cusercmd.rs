///
/// Credits to the Yuki project
///     https://github.com/Proximyst/Yuki/blob/master/src/sdk/defs/c_usercmd.rs
///


use crate::utils::math;

#[derive(Clone)]
#[repr(C)]
pub struct CUserCMD {
    pub destructor: *const *const fn(),
    pub command_number: i32,
    pub tick_count: i32,
    pub view_angles: math::vec::Vec3,
    pub aim_direction: math::vec::Vec2,
    pub forward_move: f32,
    pub side_move: f32,
    pub up_move: f32,
    pub buttons: u32,
    pub impulse: u8,
    pub weapon_select: i32,
    pub weapon_subtype: i32,
    pub random_seed: i32,
    pub mouse_dx: i16,
    pub mouse_dy: i16,
    pub has_been_predicted: bool,
    pub pad: [u8; 0x18],
}
