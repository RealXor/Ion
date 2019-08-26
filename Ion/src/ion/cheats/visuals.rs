use crate::ion::*;
use crate::ion::sdk::*;
use crate::ion::sdk::surface::Color;
use crate::vmt::*;

pub fn draw_visuals() {
    let local_optional = get_local_player();

    if local_optional.is_none() {
        return;
    }

    let local = local_optional.unwrap().to_owned();

    for player_ref in get_all_players().iter_mut() {
        let mut player = *player_ref;

        if local.get_team_num() == player.get_team_num() {
            continue;
        }

        if !player.get_health() > 1 {
            continue;
        }

        if player.is_dormant() {
            continue;
        }

        if !player.is_alive() {
            continue;
        }

        let head_w2s = world_to_screen(player.get_bone_pos(8));
        let origin_w2s = world_to_screen(player.get_origin());

        if !head_w2s.is_some() || !origin_w2s.is_some() {
            continue;
        }

        let height: i32 = (origin_w2s.unwrap().y - head_w2s.unwrap().y) as i32;
        let width = height / 2;

        let x1: i32 = (head_w2s.unwrap().x - (width / 2) as f32) as i32;
        let y1: i32 = head_w2s.unwrap().y as i32;
        let w: i32 = width;
        let h: i32 = height;

        draw_box(x1, y1, w, h, Color::new_rgb(255, 0, 0));

    }
}

fn draw_box(x: i32, y: i32, w: i32, h: i32, clr: Color) {
    interfaces.lock().unwrap().vgui_surface.set_draw_color(clr);
    interfaces.lock().unwrap().vgui_surface.draw_outlined_rect(x, y, x + w, y + h);

    interfaces.lock().unwrap().vgui_surface.set_draw_color(Color::new_rgb(0, 0, 0));
    interfaces.lock().unwrap().vgui_surface.draw_outlined_rect(x - 1, y - 1, x + w + 1, y + h + 1);
    interfaces.lock().unwrap().vgui_surface.draw_outlined_rect(x + 1, y + 1, x + w - 1, y + h - 1);
}