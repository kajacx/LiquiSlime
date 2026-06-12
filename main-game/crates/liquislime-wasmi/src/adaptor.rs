use std::ptr::NonNull;

use liquislime_core::*;
use wasmi::{Caller, Engine, Instance, Linker, Module, Store};
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

        linker
            .func_wrap(
                "wasi_snapshot_preview1",
                "adapter_close_badfd",
                |_caller: Caller<_>, _val: i32| {
                    println!("adapter_close_badfd called: {_val}");
                    0
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "wasi:io/poll@0.2.0",
                "[resource-drop]pollable",
                |_caller: Caller<_>, _val: i32| {
                    println!("[resource-drop]pollable called: {_val}");
                    ()
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "wasi:io/streams@0.2.0",
                "[resource-drop]input-stream",
                |_caller: Caller<_>, _val: i32| {
                    println!("[resource-drop]input-stream called: {_val}");
                    ()
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "wasi:io/streams@0.2.0",
                "[resource-drop]output-stream",
                |_caller: Caller<_>, _val: i32| {
                    println!("[resource-drop]output-stream called: {_val}");
                    ()
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "wasi:sockets/udp@0.2.0",
                "[resource-drop]udp-socket",
                |_caller: Caller<_>, _val: i32| {
                    println!("[resource-drop]udp-socket called: {_val}");
                    ()
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "wasi:sockets/udp@0.2.0",
                "[resource-drop]incoming-datagram-stream",
                |_caller: Caller<_>, _val: i32| {
                    println!("[resource-drop]incoming-datagram-stream called: {_val}");
                    ()
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "wasi:sockets/udp@0.2.0",
                "[resource-drop]outgoing-datagram-stream",
                |_caller: Caller<_>, _val: i32| {
                    println!("[resource-drop]outgoing-datagram-stream called: {_val}");
                    ()
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "wasi:sockets/tcp@0.2.0",
                "[resource-drop]tcp-socket",
                |_caller: Caller<_>, _val: i32| {
                    println!("[resource-drop]tcp-socket called: {_val}");
                    ()
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "wasi:sockets/tcp@0.2.0",
                "[method]tcp-socket.finish-connect",
                |_caller: Caller<_>, _val1: i32, _val2: i32| {
                    println!("[method]tcp-socket.finish-connect called: {_val1},  {_val2}");
                    ()
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "wasi:io/poll@0.2.0",
                "poll",
                |_caller: Caller<_>, _val1: i32, _val2: i32, _val3: i32| {
                    println!("poll called: {_val1},  {_val2},  {_val3}");
                    ()
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "wasi:io/streams@0.2.0",
                "[method]input-stream.subscribe",
                |_caller: Caller<_>, _val: i32| {
                    println!("[method]input-stream.subscribe called: {_val}");
                    0
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "wasi:io/streams@0.2.0",
                "[method]output-stream.subscribe",
                |_caller: Caller<_>, _val: i32| {
                    println!("[method]output-stream.subscribe called: {_val}");
                    0
                },
            )
            .unwrap();

        linker
            .func_wrap(
                "wasi:clocks/monotonic-clock@0.2.0",
                "subscribe-duration",
                |_caller: Caller<_>, _val: i64| {
                    println!("[method]subscribe-duration called: {_val}");
                    0
                },
            )
            .unwrap();

        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("TODO: Failed to create instance from module");

        Self { store, instance }
    }
}

impl BehaviourAdaptor for WasmiAdaptor {
    fn update(&mut self, game_interaction: &mut GameInteraction, time_passed: TimeInterval) {
        unsafe {
            let game_interaction = NonNull::from_mut(game_interaction.with_static_lifetime());
            self.store.data_mut().game_interaction = Some(game_interaction);
        }

        // self.instance
        //     .get_export(&self.store, "update")
        //     .expect("TODO: Failed to find 'update' export")
        //     .into_func()
        //     .expect("TODO: func")
        //     .typed::<f64, ()>(&self.store)
        //     .expect("TODO: typed")
        //     .call(&mut self.store, time_passed.to_seconds())
        //     .expect("TODO: Failed to invoke 'update' export");

        println!("Calling _start");

        self.instance
            .get_export(&self.store, "_start")
            .expect("TODO: Failed to find '_start' export")
            .into_func()
            .expect("TODO: func")
            .typed::<(), ()>(&self.store)
            .expect("TODO: typed")
            .call(&mut self.store, ())
            .expect("TODO: Failed to invoke '_start' export");

        self.store.data_mut().game_interaction = None;
    }
}
