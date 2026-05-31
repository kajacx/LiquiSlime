use liquislime_core::{GameState, Position};
use macroquad::prelude::*;

mod slime_renderer;

pub fn render_game(state: &GameState) {
    clear_background(LIGHTGRAY);
    set_camera(&create_macroquad_camera(&state.screen));

    for (index, faction) in state.factions.iter().enumerate() {
        slime_renderer::draw_slime_grid(
            &state.grids.grids[index],
            0.0,
            0.0,
            1.0,
            parse_color(faction.color()),
        );
    }
}

fn create_macroquad_camera(screen: &liquislime_core::Screen) -> Camera2D {
    let mut vp_cam = Camera2D::default();

    // `vp_cam.target` is just the center of the screen in world coordinates
    let center_of_screen = screen.screen_position_to_world(screen.size / 2.0);

    vp_cam.target.x = center_of_screen.x;
    vp_cam.target.y = center_of_screen.y;

    /*
     * `vp_cam.zoom` is very strange, `z` zoom means one unit in world coordinates takes `z` of HALF of the screen.
     * For example, 0.05 zoom means one unit takes 1/20 of the half of the screen.
     */
    let unit_vector = Position::new(1.0, 1.0);
    let in_screen = screen.world_vector_to_screen(unit_vector);
    let relative = in_screen / screen.size * 2.0;

    vp_cam.zoom.x = relative.x;
    vp_cam.zoom.y = relative.y;

    vp_cam
}

fn parse_color(color: liquislime_core::Color) -> Color {
    Color::new(
        color.r as f32 / 255.0,
        color.g as f32 / 255.0,
        color.b as f32 / 255.0,
        1.0,
    )
}
