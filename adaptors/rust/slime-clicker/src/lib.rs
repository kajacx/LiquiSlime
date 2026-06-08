use liquislime_adaptor_rust::{BehaviourAdaptor, FactionId, GameApi, InputKey};

struct ClickerAdaptor;

impl BehaviourAdaptor for ClickerAdaptor {
    fn new() -> Self {
        ClickerAdaptor
    }

    fn update(&mut self, delta_time: f64) {
        let pos = GameApi::get_mouse_world_position();
        if GameApi::is_key_pressed(InputKey::LeftMouse) {
            println!("Mouse left button is pressed!");
            GameApi::add_slime_amount(pos.to_tile_position(), FactionId::new(0), 100_000.0);
        }
    }
}

liquislime_adaptor_rust::export_adaptor!(ClickerAdaptor, ClickerAdaptor);

// static ADAPTOR: ::liquislime_adaptor_rust::TryLock<ClickerAdaptor> =
//     ::liquislime_adaptor_rust::TryLock::new(ClickerAdaptor);

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn update(time_passed: f64) {
//     ADAPTOR.try_lock().unwrap().update(time_passed as f64);
// }
