use crate::{component_adaptor::ResultType, wasip2::AddToLinkerParams};
use waclay::*;

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    let instance = params
        .linker
        .define_instance("wasi:clocks/monotonic-clock@0.2.6".try_into()?)?;

    instance.define_func(
        "now",
        TypedFunc::new(&mut params.store, |_, ()| {
            println!("[STUB] now called");
            Ok(0u64)
        })
        .func(),
    )?;

    instance.define_func(
        "subscribe-instant",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::U64],
                [ValueType::Own(params.resources.io.poll.pollable.clone())],
            ),
            |_, _, _| {
                println!("[STUB] subscribe-instant called");
                Ok(())
            },
        ),
    )?;

    instance.define_func(
        "subscribe-duration",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::U64],
                [ValueType::Own(params.resources.io.poll.pollable.clone())],
            ),
            |_, _, _| {
                println!("[STUB] subscribe-duration called");
                Ok(())
            },
        ),
    )?;

    Ok(())
}
