use crate::{component_adaptor::ResultType, wasip2::AddToLinkerParams};
use waclay::*;

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    let instance = params
        .linker
        .define_instance("wasi:cli/terminal-stderr@0.2.6".try_into()?)?;

    instance.define_func(
        "get-terminal-stderr",
        Func::new(
            &mut params.store,
            FuncType::new(
                [],
                [ValueType::Option(OptionType::new(ValueType::Own(
                    params.resources.cli.terminal_output.terminal_output.clone(),
                )))],
            ),
            |_, _, _| {
                println!("[STUB] get-terminal-stderr called");
                Ok(())
            },
        ),
    )?;

    Ok(())
}
