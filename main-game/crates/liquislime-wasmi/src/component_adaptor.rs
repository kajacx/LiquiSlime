use std::{
    ptr::NonNull,
    time::{Duration, SystemTime},
};

use liquislime_core::*;
use waclay_wasi::{
    Component, Engine, Func, FuncType, Instance, Linker, Store, ValueType, WasiP2Ctx,
    WasiP2MonotonicClock, WasiP2WallClock,
};

pub struct WasmiComponentAdaptor {
    store: Store<StoreData, wasmi_runtime_layer::Engine>,
    instance: Instance,
}

pub(crate) struct StoreData {
    pub(crate) game_interaction: Option<NonNull<GameInteraction<'static>>>,
    wasi_ctx: waclay_wasi::WasiP2Ctx,
}

impl waclay_wasi::AsWasiP2Ctx for StoreData {
    fn as_wasi(&self) -> &waclay_wasi::WasiP2Ctx {
        &self.wasi_ctx
    }

    fn as_wasi_mut(&mut self) -> &mut waclay_wasi::WasiP2Ctx {
        &mut self.wasi_ctx
    }
}

impl WasmiComponentAdaptor {
    pub fn new(bytes: &[u8]) -> Self {
        // 1. 1. 2026
        let start_time = SystemTime::UNIX_EPOCH + Duration::from_secs(1767225600);

        let store_data = StoreData {
            game_interaction: None,
            wasi_ctx: WasiP2Ctx::from_builder(|ctx| {
                ctx.inherit_stdout()
                    .inherit_stderr()
                    .set_rng(1337)
                    .set_wall_clock(Box::new(LiquislimeClock::new(start_time)))
                    .set_monotonic_clock(Box::new(LiquislimeClock::new(start_time)));
            }),
        };

        let engine = Engine::new(wasmi_runtime_layer::Engine::default());

        let mut store = Store::new(&engine, store_data);

        let module = Component::new(store.engine(), bytes)
            .expect("TODO: Failed to create component from bytes");

        let mut linker = Linker::default();

        waclay_wasi::add_to_linker(&mut linker, &mut store).unwrap();

        let linker_instance = linker
            .define_instance("example:component/operations".try_into().unwrap())
            .unwrap();

        linker_instance
            .define_func(
                "add",
                Func::new(
                    &mut store,
                    FuncType::new([ValueType::S32, ValueType::S32], [ValueType::S32]),
                    |_, _, _| Ok(()),
                ),
            )
            .unwrap();

        let instance = linker
            .instantiate(&mut store, &module)
            .expect("TODO: Failed to create instance from module");

        Self { store, instance }
    }
}

impl BehaviourAdaptor for WasmiComponentAdaptor {
    fn update(&mut self, game_interaction: &mut GameInteraction, time_elapsed: TimeInterval) {
        let wasi_ctx = &mut self.store.data_mut().wasi_ctx;

        wasi_ctx
            .wall_clock
            .as_any_mut()
            .downcast_mut::<LiquislimeClock>()
            .unwrap()
            .time_elapsed += time_elapsed;

        wasi_ctx
            .monotonic_clock
            .as_any_mut()
            .downcast_mut::<LiquislimeClock>()
            .unwrap()
            .time_elapsed += time_elapsed;

        unsafe {
            let game_interaction = NonNull::from_mut(game_interaction.with_static_lifetime());
            self.store.data_mut().game_interaction = Some(game_interaction);
        }

        let update = self
            .instance
            .exports()
            .instance(&"example:component/operations".try_into().unwrap())
            .expect("get example:component instance")
            .func("add")
            .expect("get add function")
            .typed::<(i32, i32), i32>()
            .expect("add function as typed");

        // println!("Found update func: {update:?}");
        let result = update
            .call(&mut self.store, (5, 6))
            .expect("call add function");

        println!("ADD RESULT: {result}");

        self.store.data_mut().game_interaction = None;
    }
}

#[derive(Debug)]
struct LiquislimeClock {
    pub start_time: SystemTime,
    pub time_elapsed: TimeInterval,
}

impl LiquislimeClock {
    pub fn new(start_time: SystemTime) -> Self {
        Self {
            start_time,
            time_elapsed: TimeInterval::from_seconds(0.0),
        }
    }
}

impl WasiP2WallClock for LiquislimeClock {
    fn now(&mut self) -> waclay_wasi::bindings::Datetime {
        let date = self.start_time + Duration::from_secs_f64(self.time_elapsed.to_seconds());
        let duration = date.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        waclay_wasi::bindings::Datetime {
            seconds: duration.as_secs(),
            nanoseconds: duration.subsec_nanos(),
        }
    }
}

impl WasiP2MonotonicClock for LiquislimeClock {
    fn now(&mut self) -> waclay_wasi::bindings::Instant {
        (self.time_elapsed.to_seconds() * 1_000_000_000.0) as u64
    }
}
