use liquislime_core::{
    Faction, GameState, Screen, SlimeAmount, SlimeGrid, TilePosition, TimeInterval,
};
use macroquad::{input, prelude::*};

use crate::input_helper::InputHelper;

mod input_helper;
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

    // println!("{:?}", std::env::current_dir().unwrap());
    // let texture = load_texture("assets/lucy.png").await.unwrap();

    // let mut error = None;

    loop {
        // let result = std::panic::catch_unwind(|| {
        update(&mut state);
        render::render_game(&state);
        // });

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn update(state: &mut GameState) {
    state.update(TimeInterval::from_seconds(get_frame_time() as f64));

    if input::is_mouse_button_pressed(MouseButton::Left) {
        #[allow(unused_must_use)]
        state.grids.try_add_amount(
            state.factions[0].id(),
            InputHelper::get_mouse_tile_position(&state.screen),
            SlimeAmount::from_integer(1000000),
        );
    }

    if input::is_mouse_button_down(MouseButton::Right) {
        #[allow(unused_must_use)]
        state.grids.try_add_amount(
            state.factions[1].id(),
            InputHelper::get_mouse_tile_position(&state.screen),
            SlimeAmount::from_integer(10000),
        );

        // let mouse_pos = input::mouse_position();
        // hero_pos = Vec2::new(mouse_pos.0, mouse_pos.1);
    }
}
