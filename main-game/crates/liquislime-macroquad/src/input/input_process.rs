use super::{input_constants, InputHelper};
use liquislime_core::{GameState, Position};

pub fn process_inputs(state: &mut GameState) {
    process_camera_pan_keyboard(state);
    process_camera_zoom(state);
}

fn process_camera_pan_keyboard(state: &mut GameState) {
    if InputHelper::is_key_down(liquislime_core::InputKey::Left) {
        state
            .screen
            .camera
            .pan_by(Position::new(-input_constants::PAN_SENSITIVITY, 0.0));
    }

    if InputHelper::is_key_down(liquislime_core::InputKey::Right) {
        state
            .screen
            .camera
            .pan_by(Position::new(input_constants::PAN_SENSITIVITY, 0.0));
    }

    if InputHelper::is_key_down(liquislime_core::InputKey::Up) {
        state
            .screen
            .camera
            .pan_by(Position::new(0.0, -input_constants::PAN_SENSITIVITY));
    }

    if InputHelper::is_key_down(liquislime_core::InputKey::Down) {
        state
            .screen
            .camera
            .pan_by(Position::new(0.0, input_constants::PAN_SENSITIVITY));
    }
}

fn process_camera_zoom(state: &mut GameState) {
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
