use crate::{component_adaptor::ResultType, wasip2::AddToLinkerParams};
use waclay::*;

pub struct PollResources {
    pub pollable: ResourceType,
}

pub struct PollableResource {}

pub fn register_resources(linker: &mut Linker) -> ResultType<PollResources> {
    let instance = linker.define_instance("wasi:io/poll@0.2.6".try_into()?)?;

    let pollable =
        ResourceType::new::<PollableResource>(Some(TypeIdentifier::new("Pollable hello", None)));
    instance.define_resource("pollable", pollable.clone())?;

    Ok(PollResources { pollable })
}

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    let instance = params
        .linker
        .instance_mut(&"wasi:io/poll@0.2.6".try_into()?)
        .expect("TODO: get the poll instance");

    instance.define_func(
        "[method]pollable.block",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::Borrow(params.resources.io.poll.pollable.clone())],
                // [ValueType::Borrow(ResourceType::new::<PollableResource>(
                //     None,
                // ))],
                [],
            ),
            |_, _, _| {
                println!("[STUB] [method]pollable.block called");
                Ok(())
            },
        ),
    )?;

    instance.define_func(
        "poll",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::List(ListType::new(ValueType::Borrow(
                    params.resources.io.poll.pollable.clone(),
                )))],
                [ValueType::List(ListType::new(ValueType::U32))],
            ),
            |_, _, _| {
                println!("[STUB] poll called");
                Ok(())
            },
        ),
    )?;

    Ok(())
}
