use crate::{component_adaptor::ResultType, wasip2::AddToLinkerParams};
use waclay::*;

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    let instance = params
        .linker
        .define_instance("wasi:filesystem/preopens@0.2.6".try_into()?)?;

    instance.define_func(
        "get-directories",
        Func::new(
            &mut params.store,
            FuncType::new(
                [],
                [ValueType::List(ListType::new(ValueType::Tuple(
                    TupleType::new(
                        None,
                        [
                            ValueType::Own(params.resources.filesystem.types.descriptor.clone()),
                            ValueType::String,
                        ],
                    ),
                )))],
            ),
            |_, _, _| {
                println!("[STUB] now called");
                Ok(())
            },
        ),
    )?;

    Ok(())
}
