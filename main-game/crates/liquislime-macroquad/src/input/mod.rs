mod input_helper;
mod input_process;

pub use input_helper::*;
use liquislime_core::*;

pub fn process_inputs(state: &mut liquislime_core::GameState) {
    input_process::process_inputs(state);
    example_interaction(state);
}

fn example_interaction(state: &mut liquislime_core::GameState) {
    if InputHelper::is_key_pressed(liquislime_core::InputKey::LeftMouse) {
        #[allow(unused_must_use)]
        state.grids.try_add_amount(
            state.factions[0].id(),
            InputHelper::get_mouse_tile_position(&state.screen),
            SlimeAmount::from_integer(1000000),
        );
    }

    if InputHelper::is_key_down(liquislime_core::InputKey::RightMouse) {
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
