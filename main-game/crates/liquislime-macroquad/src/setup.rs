pub fn setup_panic_hook() {
    #[cfg(not(target_arch = "wasm32"))]
    return;

    // Not actually unreachable, but the compiler only checks the sys target
    #[allow(unreachable_code)]
    std::panic::set_hook(Box::new(|panic_info| {
        let msg = panic_info
            .payload_as_str()
            .unwrap_or("Unknown panic message");

        let location = panic_info.location();
        if let Some(location) = location {
            macroquad::logging::error!("Panic at {}: {}", location, msg);
        } else {
            macroquad::logging::error!("Panic at unknown location: {}", msg);
        }
    }));
}
