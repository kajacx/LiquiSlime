use super::InputHelper;
use liquislime_core::{GameState, Position};

pub fn process_inputs(state: &mut GameState) {
    process_camera_pan_keyboard(state);
    process_camera_zoom(state);
}

fn process_camera_pan_keyboard(state: &mut GameState) {
    if InputHelper::is_key_down(liquislime_core::InputKey::Left) {
        state.screen.camera.pan_by(Position::new(-1.0, 0.0));
    }

    if InputHelper::is_key_down(liquislime_core::InputKey::Right) {
        state.screen.camera.pan_by(Position::new(1.0, 0.0));
    }

    if InputHelper::is_key_down(liquislime_core::InputKey::Up) {
        state.screen.camera.pan_by(Position::new(0.0, -1.0));
    }

    if InputHelper::is_key_down(liquislime_core::InputKey::Down) {
        state.screen.camera.pan_by(Position::new(0.0, 1.0));
    }
}

fn process_camera_zoom(state: &mut GameState) {
    if InputHelper::is_key_pressed(liquislime_core::InputKey::MouseWheelUp) {
        state.screen.camera.change_zoom(-1);
    }

    if InputHelper::is_key_pressed(liquislime_core::InputKey::MouseWheelDown) {
        state.screen.camera.change_zoom(1);
    }
}
