use crate::ion::*;
use crate::ion::sdk::definitions::entity::CEntity;
use crate::ion::sdk::definitions::recvprop::EPropType;
use crate::utils::math::vec::{Vec2, Vec3};

/**
    Consider this a 'hub' of sorts of modules that are used to change / access
    structures / classes that are from CS:GO itself.
*/

pub mod glow;
pub mod engine;
pub mod surface;
pub mod panel;
pub mod entitylist;
pub mod client;
pub mod debugoverlay;

pub mod netvar;
pub mod interfaces;
pub mod definitions;
pub mod hook;

pub fn get_local_player() -> Option<definitions::entity::CEntity> {
    let interfaces = INTERFACES.lock().unwrap();

    let local_id = interfaces.engine.get_local_player();

    if local_id == 0 {
        return None;
    }

    unsafe {
        Some(CEntity::from_raw(interfaces.entity_list.get_entity_by_id(local_id)))
    }
}

pub fn world_to_screen(input: Vec3) -> Option<Vec3> {
    INTERFACES.lock().unwrap().debug_overlay.world_to_screen(&input)
}

pub fn get_entity_by_id(id: i32) -> *mut usize {
    INTERFACES.lock().unwrap().entity_list.get_entity_by_id(id)
}

pub fn get_highest_entity_index() -> i32 {
    INTERFACES.lock().unwrap().entity_list.get_highest_ent_idx()
}

pub fn get_all_players() -> impl Iterator<Item=CEntity> {
    (0..get_highest_entity_index())
        .map(|i| unsafe { CEntity::from_raw(get_entity_by_id(i)) })
        .filter(|&e| {
            !e.is_empty()
                && e.is_player()
                && e.is_alive()
                && !e.is_dormant()
        })
}