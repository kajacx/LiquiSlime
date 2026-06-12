use std::ptr::NonNull;

use liquislime_core::*;
use waclay::{Component, Engine, Instance, Linker, Store};

pub struct WasmiComponentAdaptor {
    store: Store<StoreData, wasmi_runtime_layer::Engine>,
    instance: Instance,
}

pub(crate) struct StoreData {
    pub(crate) game_interaction: Option<NonNull<GameInteraction<'static>>>,
    // pub(crate) ctx: WasiCtx,
}

impl WasmiComponentAdaptor {
    pub fn new(bytes: &[u8]) -> Self {
        let store_data = StoreData {
            game_interaction: None,
        };

        let engine = Engine::new(wasmi_runtime_layer::Engine::default());

        let mut store = Store::new(&engine, store_data);

        let module = Component::new(store.engine(), bytes)
            .expect("TODO: Failed to create component from bytes");

        let linker = Linker::default();

        let instance = linker
            .instantiate(&mut store, &module)
            .expect("TODO: Failed to create instance from module");

        Self { store, instance }
    }
}

impl BehaviourAdaptor for WasmiComponentAdaptor {
    fn update(&mut self, game_interaction: &mut GameInteraction, time_passed: TimeInterval) {
        unsafe {
            let game_interaction = NonNull::from_mut(game_interaction.with_static_lifetime());
            self.store.data_mut().game_interaction = Some(game_interaction);
        }

        let update = self
            .instance
            .exports()
            .root()
            .func("update")
            .expect("Get update func");

        println!("Found update func: {update:?}");

        self.store.data_mut().game_interaction = None;
    }
}
