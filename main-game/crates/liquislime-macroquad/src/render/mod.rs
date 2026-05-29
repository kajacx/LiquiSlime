use liquislime_core::GameState;
use macroquad::prelude::*;

mod slime_renderer;

pub fn render_game(state: &GameState) {
    clear_background(LIGHTGRAY);

    for (index, faction) in state.factions.iter().enumerate() {
        slime_renderer::draw_slime_grid(
            &state.grids.grids[index],
            0.0,
            0.0,
            1.0 / state.screen.camera.zoom,
            parse_color(faction.color()),
        );
    }
}

fn parse_color(color: liquislime_core::Color) -> Color {
    Color::new(
        color.r as f32 / 255.0,
        color.g as f32 / 255.0,
        color.b as f32 / 255.0,
        1.0,
    )
}
