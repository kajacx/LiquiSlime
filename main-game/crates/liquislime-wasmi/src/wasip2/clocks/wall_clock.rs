use waclay::*;

use crate::{
    component_adaptor::ResultType,
    wasip2::{AddToLinkerParams, WasiP2TypesBuilder},
};

pub fn register_types(builder: &mut WasiP2TypesBuilder) -> ResultType<()> {
    builder.datetime.set_fn(|_, _| {
        Ok(ValueType::Record(RecordType::new(
            None,
            [("seconds", ValueType::U64), ("nanoseconds", ValueType::U32)],
        )?))
    })?;

    Ok(())
}

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    let instance = params
        .linker
        .define_instance("wasi:clocks/wall-clock@0.2.6".try_into()?)?;

    instance.define_func(
        "now",
        Func::new(
            &mut params.store,
            FuncType::new([], [params.types.datetime.clone()]),
            |_, _, _| {
                println!("[STUB] now called");
                Ok(())
            },
        ),
    )?;

    Ok(())
}
