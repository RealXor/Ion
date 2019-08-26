use crate::ion::*;
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
    let local_id = INTERFACES.lock().unwrap().engine.get_local_player();

    if local_id == 0 {
        return None;
    }

    unsafe {
        Some(definitions::entity::CEntity::from_raw(INTERFACES.lock().unwrap().entity_list.get_entity_by_id(local_id)))
    }
}

pub fn world_to_screen(input: Vec3) -> Option<Vec3> {
    INTERFACES.lock().unwrap().debug_overlay.world_to_screen(&input)
}

pub fn get_all_players() -> Vec<definitions::entity::CEntity> {
    let mut players: Vec<definitions::entity::CEntity> = vec![];

    let max = INTERFACES.lock().unwrap().entity_list.get_highest_ent_idx();

    for i in 0..max {
        let entity_ptr: *mut usize = INTERFACES.lock().unwrap().entity_list.get_entity_by_id(i);

        if entity_ptr.is_null() {
            continue;
        }

        let entity = unsafe {
            definitions::entity::CEntity::from_raw(entity_ptr)
        };

        if entity.is_player() {
            if !entity.get_health() > 1 {
                continue;
            }

            if entity.is_dormant() {
                continue;
            }

            if !entity.is_alive() {
                continue;
            }

            players.push(entity)
        }
    }

    players
}