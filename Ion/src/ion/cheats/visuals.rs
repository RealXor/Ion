
use crate::vmt::*;
use crate::ion::*;

use crate::ion::sdk::surface::Color;
use crate::ion::sdk::*;

pub fn draw_visuals() {
    let local = get_local_player();

    if local.is_none() {
        return;
    }

    for player_ref in get_all_players().iter_mut() {
        let mut player = *player_ref;


    }
}