use crate::{component_adaptor::ResultType, wasip2::AddToLinkerParams};
use waclay::*;

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    let instance = params
        .linker
        .define_instance("wasi:cli/stdin@0.2.6".try_into()?)?;

    instance.define_func(
        "get-stdin",
        Func::new(
            &mut params.store,
            FuncType::new(
                [],
                [ValueType::Own(
                    params.resources.io.streams.input_stream.clone(),
                )],
            ),
            |_, _, _| {
                println!("[STUB] get-stdin called");
                Ok(())
            },
        ),
    )?;

    Ok(())
}
