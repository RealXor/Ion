use crate::ion::*;
use crate::ion::sdk::definitions::recvprop::e_prop_type;
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

pub fn get_local_player() -> Option<definitions::entity::c_entity> {
    let local_id = interfaces.lock().unwrap().engine.get_local_player();

    if local_id == 0 {
        return None;
    }

    unsafe {
        Some(definitions::entity::c_entity::from_raw(interfaces.lock().unwrap().entity_list.get_entity_by_id(local_id)))
    }
}

pub fn world_to_screen(input: Vec3) -> Option<Vec3> {
    interfaces.lock().unwrap().debug_overlay.world_to_screen(&input)
}

pub fn get_all_players() -> Vec<definitions::entity::c_entity> {
    let mut players: Vec<definitions::entity::c_entity> = vec![];

    let max = interfaces.lock().unwrap().entity_list.get_highest_ent_idx();

    for i in 0..max {
        let entity_ptr: *mut usize = interfaces.lock().unwrap().entity_list.get_entity_by_id(i);

        if entity_ptr.is_null() {
            continue;
        }

        let entity = unsafe {
            definitions::entity::c_entity::from_raw(entity_ptr)
        };

        if entity.is_player() {
            players.push(entity)
        }
    }

    players
}