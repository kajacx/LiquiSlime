use std::ptr::NonNull;

use liquislime_core::*;
use wasmi::{Engine, Instance, Linker, Module, Store};
use wasmi_wasi::WasiCtx;

use crate::api::add_imported_game_functions;

pub struct WasmiAdaptor {
    store: Store<StoreData>,
    instance: Instance,
}

pub(crate) struct StoreData {
    pub(crate) game_interaction: Option<NonNull<GameInteraction<'static>>>,
    pub(crate) ctx: WasiCtx,
}

impl WasmiAdaptor {
    pub fn new(bytes: &[u8]) -> Self {
        let ctx = wasmi_wasi::WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build();

        let store_data = StoreData {
            game_interaction: None,
            ctx,
        };

        let mut store = Store::new(&Engine::default(), store_data);

        let module =
            Module::new(store.engine(), bytes).expect("TODO: Failed to create module from bytes");

        let mut linker = Linker::<StoreData>::new(&store.engine());

        wasmi_wasi::add_to_linker(&mut linker, |store: &mut StoreData| &mut store.ctx)
            .expect("TODO: Failed to add WASI to linker");

        add_imported_game_functions(&mut linker).expect("TODO: linker error");

        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("TODO: Failed to create instance from module");

        let start = instance.get_export(&store, "_start");
        if let Some(start) = start {
            start
                .into_func()
                .expect("TODO: func")
                .typed::<(), ()>(&store)
                .expect("TODO: typed")
                .call(&mut store, ())
                .expect("TODO: Failed to invoke '_start' export");
        }

        Self { store, instance }
    }
}

impl BehaviourAdaptor for WasmiAdaptor {
    fn update(&mut self, game_interaction: &mut GameInteraction, time_elapsed: TimeInterval) {
        unsafe {
            let game_interaction = NonNull::from_mut(game_interaction.with_static_lifetime());
            self.store.data_mut().game_interaction = Some(game_interaction);
        }

        println!("Calling update");

        let result = self
            .instance
            .get_export(&self.store, "update")
            .expect("TODO: Failed to find 'update' export")
            .into_func()
            .expect("TODO: func")
            .typed::<f64, ()>(&self.store)
            .expect("TODO: typed")
            .call(&mut self.store, time_elapsed.to_seconds());

        if let Err(error) = result {
            println!("Error when calling update: {error}");
        }

        self.store.data_mut().game_interaction = None;
    }
}
