use crate::{component_adaptor::ResultType, wasip2::AddToLinkerParams};
use waclay::*;

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    let instance = params
        .linker
        .define_instance("wasi:cli/environment@0.2.6".try_into()?)?;

    instance.define_func(
        "get-environment",
        TypedFunc::new(
            &mut params.store,
            |_, ()| -> Result<Vec<(String, String)>, _> {
                println!("[STUB] Called 'get-environment'");
                Ok(vec![])
            },
        )
        .func(),
    )?;

    instance.define_func(
        "get-arguments",
        TypedFunc::new(&mut params.store, |_, ()| -> Result<Vec<String>, _> {
            println!("[STUB] Called 'get-arguments'");
            Ok(vec!["Csharp Adaptor".into()])
        })
        .func(),
    )?;

    Ok(())
}
