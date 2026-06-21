use std::{error::Error, ptr::NonNull};

use liquislime_core::*;
use waclay::{Component, Engine, Func, FuncType, Instance, Linker, Store, ValueType};

use crate::wasip2::{add_to_linker, LoopingRng};

pub struct WasmiComponentAdaptor {
    store: Store<StoreData, wasmi_runtime_layer::Engine>,
    instance: Instance,
}

pub(crate) struct StoreData {
    pub(crate) game_interaction: Option<NonNull<GameInteraction<'static>>>,
    pub rng: LoopingRng,
}

pub(crate) type StoreType = Store<StoreData, wasmi_runtime_layer::Engine>;
pub(crate) type ResultType<T> = Result<T, Box<dyn Error>>;

impl WasmiComponentAdaptor {
    pub fn new(bytes: &[u8]) -> Self {
        let store_data = StoreData {
            game_interaction: None,
            rng: LoopingRng::new(),
        };

        let engine = Engine::new(wasmi_runtime_layer::Engine::default());

        let mut store = Store::new(&engine, store_data);

        let module = Component::new(store.engine(), bytes)
            .expect("TODO: Failed to create component from bytes");

        let mut linker = Linker::default();

        // add_wasip2_to_linker(&mut store, &mut linker).unwrap();

        add_to_linker(&mut store, &mut linker).unwrap();

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
    fn update(&mut self, game_interaction: &mut GameInteraction, _time_elapsed: TimeInterval) {
        unsafe {
            let game_interaction = NonNull::from_mut(game_interaction.with_static_lifetime());
            self.store.data_mut().game_interaction = Some(game_interaction);
        }

        // let update = self
        //     .instance
        //     .exports()
        //     .instance(&"wasi:cli/run@0.2.0".try_into().unwrap())
        //     .expect("get wasi cli instance")
        //     .func("run")
        //     .expect("get run function")
        //     .typed::<(), Result<(), ()>>()
        //     .expect("main run function as typed");

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
