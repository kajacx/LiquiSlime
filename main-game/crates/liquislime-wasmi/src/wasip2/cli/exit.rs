use crate::{component_adaptor::ResultType, wasip2::AddToLinkerParams};
use waclay::*;

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    let instance = params
        .linker
        .define_instance("wasi:cli/exit@0.2.6".try_into()?)?;

    instance.define_func(
        "exit",
        TypedFunc::new(&mut params.store, |_, params: (Result<(), ()>,)| {
            let (status,) = params;
            println!("[STUB] Called 'exit' with status {status:?}");
            Ok(())
        })
        .func(),
    )?;

    Ok(())
}
