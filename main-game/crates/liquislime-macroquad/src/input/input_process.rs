use super::{input_constants, InputHelper};
use liquislime_core::*;

pub fn _process_inputs(state: &mut GameState) {
    _process_camera_pan_keyboard(state);
    _process_camera_zoom(state);
}

fn _process_camera_pan_keyboard(_state: &mut GameState) {}

fn _process_camera_zoom(state: &mut GameState) {
    let world_before_zoom = InputHelper::get_mouse_world_position(&state.screen);
    let screen_before_zoom = InputHelper::get_mouse_screen_position();

    if InputHelper::is_key_pressed(liquislime_core::InputKey::MouseWheelUp) {
        state
            .screen
            .camera
            .change_zoom(1.0 / input_constants::ZOOM_SENSITIVITY);
        state
            .screen
            .match_screen_and_world(screen_before_zoom, world_before_zoom);
    }

    if InputHelper::is_key_pressed(liquislime_core::InputKey::MouseWheelDown) {
        state
            .screen
            .camera
            .change_zoom(input_constants::ZOOM_SENSITIVITY);
        state
            .screen
            .match_screen_and_world(screen_before_zoom, world_before_zoom);
    }
}
