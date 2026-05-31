use crate::input::InputHelper;
use liquislime_core::{Faction, GameState, Screen, SlimeAmount, TilePosition, TimeInterval};
use macroquad::prelude::*;

mod input;
mod render;
mod setup;
mod texture_atlas;

#[macroquad::main("Liquislime")]
async fn main() {
    setup::setup_panic_hook();

    let faction0 = Faction::new(0, liquislime_core::Color::new(255, 128, 0));
    let faction1 = Faction::new(1, liquislime_core::Color::new(0, 255, 64));

    let factions = vec![faction0, faction1];

    let screen = Screen::new(InputHelper::screen_size(), 0.075f32);
    let mut state = GameState::new(factions, 50, 50, screen);

    state.grids.set_amount(
        state.factions[0].id(),
        TilePosition::new(2, 4),
        SlimeAmount::from_integer(50000),
    );

    state.grids.set_amount(
        state.factions[1].id(),
        TilePosition::new(8, 5),
        SlimeAmount::from_integer(60000),
    );

    loop {
        // let result = std::panic::catch_unwind(|| {
        state.screen.size = InputHelper::screen_size();
        update(&mut state);
        render::render_game(&state);
        // });

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn update(state: &mut GameState) {
    state.update(TimeInterval::from_seconds(get_frame_time() as f64));
    input::process_inputs(state);
}
