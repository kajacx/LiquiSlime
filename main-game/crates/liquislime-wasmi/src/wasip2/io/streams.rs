use crate::{
    component_adaptor::ResultType,
    wasip2::{AddToLinkerParams, WasiP2TypesBuilder},
};
use waclay::*;

pub struct StreamResources {
    pub input_stream: ResourceType,
    pub output_stream: ResourceType,
}

pub struct InputStreamResource {}

pub struct OutputStreamResource {}

pub fn register_resources(linker: &mut Linker) -> ResultType<StreamResources> {
    let instance = linker.define_instance("wasi:io/streams@0.2.6".try_into()?)?;

    let input_stream = ResourceType::new::<InputStreamResource>(None);
    instance.define_resource("input-stream", input_stream.clone())?;

    let output_stream = ResourceType::new::<OutputStreamResource>(None);
    instance.define_resource("output-stream", output_stream.clone())?;

    Ok(StreamResources {
        input_stream,
        output_stream,
    })
}

pub fn register_types(builder: &mut WasiP2TypesBuilder) -> ResultType<()> {
    builder.stream_error.set_fn(|_, resources| {
        Ok(ValueType::Variant(VariantType::new(
            None,
            [
                VariantCase::new(
                    "last-operation-failed",
                    Some(ValueType::Own(resources.io.error.error.clone())),
                ),
                VariantCase::new("closed", None),
            ],
        )?))
    })?;

    Ok(())
}

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    let instance = params
        .linker
        .instance_mut(&"wasi:io/streams@0.2.6".try_into()?)
        .expect("TODO: get streams instance");

    instance.define_func(
        "[method]input-stream.subscribe",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::Borrow(
                    params.resources.io.streams.input_stream.clone(),
                )],
                [ValueType::Own(params.resources.io.poll.pollable.clone())],
            ),
            |_, _, _| {
                println!("[STUB] [method]input-stream.subscribe called");
                Ok(())
            },
        ),
    )?;

    instance.define_func(
        "[method]output-stream.subscribe",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::Borrow(
                    params.resources.io.streams.output_stream.clone(),
                )],
                [ValueType::Own(params.resources.io.poll.pollable.clone())],
            ),
            |_, _, _| {
                println!("[STUB] [method]output-stream.subscribe called");
                Ok(())
            },
        ),
    )?;

    let result_type = waclay::ResultType::new(
        Some(ValueType::U64),
        Some(params.types.stream_error.clone()),
    );

    instance.define_func(
        "[method]output-stream.check-write",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::Borrow(
                    params.resources.io.streams.output_stream.clone(),
                )],
                [ValueType::Result(result_type.clone())],
            ),
            move |_, _, results| {
                println!("[STUB] [method]output-stream.check-write called");

                results[0] = Value::Result(ResultValue::new(
                    result_type.clone(),
                    Result::Ok(Some(Value::U64(4 * 1024 * 1024))),
                )?);

                Ok(())
            },
        ),
    )?;

    let result_type = waclay::ResultType::new(None, Some(params.types.stream_error.clone()));

    instance.define_func(
        "[method]output-stream.write",
        Func::new(
            &mut params.store,
            FuncType::new(
                [
                    ValueType::Borrow(params.resources.io.streams.output_stream.clone()),
                    ValueType::List(ListType::new(ValueType::U8)),
                ],
                [ValueType::Result(result_type.clone())],
            ),
            move |_, params, results| {
                println!("[STUB] [method]output-stream.write called");

                let data: &[u8] = match &params[1] {
                    Value::List(list) => list.typed()?,
                    _ => panic!(),
                };

                let message = String::from_utf8_lossy(data);
                println!("yoyoyo: {message}");

                results[0] =
                    Value::Result(ResultValue::new(result_type.clone(), Result::Ok(None))?);

                Ok(())
            },
        ),
    )?;

    let result_type = waclay::ResultType::new(None, Some(params.types.stream_error.clone()));

    instance.define_func(
        "[method]output-stream.blocking-flush",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::Borrow(
                    params.resources.io.streams.output_stream.clone(),
                )],
                [ValueType::Result(result_type.clone())],
            ),
            move |_, _, results| {
                println!("[STUB] [method]output-stream.blocking-flush called");

                results[0] =
                    Value::Result(ResultValue::new(result_type.clone(), Result::Ok(None))?);

                Ok(())
            },
        ),
    )?;

    Ok(())
}
