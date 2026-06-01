use liquislime_core::*;

pub struct PanCamera;

impl BehaviourAdaptor for PanCamera {
    fn update(&mut self, interaction: &mut GameInteraction, delta_time: TimeInterval) {
        let amount =
            crate::input::input_constants::PAN_SENSITIVITY * delta_time.to_seconds() as f32;

        if interaction.is_key_down(InputKey::Left) {
            interaction.pan_camera_by(Position::new(-amount, 0.0));
        }

        if interaction.is_key_down(InputKey::Right) {
            interaction.pan_camera_by(Position::new(amount, 0.0));
        }

        if interaction.is_key_down(InputKey::Up) {
            interaction.pan_camera_by(Position::new(0.0, -amount));
        }

        if interaction.is_key_down(InputKey::Down) {
            interaction.pan_camera_by(Position::new(0.0, amount));
        }
    }
}
