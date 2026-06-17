use crate::{
    component_adaptor::ResultType,
    wasip2::{io::streams::OutputStreamResource, AddToLinkerParams},
};
use waclay::*;

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    let instance = params
        .linker
        .define_instance("wasi:cli/stdout@0.2.6".try_into()?)?;

    let output_stream_resource_type = params.resources.io.streams.output_stream.clone();

    instance.define_func(
        "get-stdout",
        Func::new(
            &mut params.store,
            FuncType::new([], [ValueType::Own(output_stream_resource_type.clone())]),
            move |caller, _, results| {
                println!("[STUB] get-stdout called");

                results[0] = Value::Own(ResourceOwn::new(
                    caller,
                    OutputStreamResource {},
                    output_stream_resource_type.clone(),
                )?);

                Ok(())
            },
        ),
    )?;

    Ok(())
}
