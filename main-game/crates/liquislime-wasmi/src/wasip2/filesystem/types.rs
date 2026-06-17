use crate::{
    component_adaptor::ResultType,
    wasip2::{AddToLinkerParams, WasiP2TypesBuilder},
};
use waclay::*;

#[allow(unused)]
pub struct TypesResources {
    pub descriptor: ResourceType,
}

struct DescriptorResource {}

pub fn register_resources(linker: &mut Linker) -> ResultType<TypesResources> {
    let instance = linker.define_instance("wasi:filesystem/types@0.2.6".try_into()?)?;

    let descriptor = ResourceType::new::<DescriptorResource>(None);
    instance.define_resource("descriptor", descriptor.clone())?;

    Ok(TypesResources { descriptor })
}

pub fn register_types(builder: &mut WasiP2TypesBuilder) -> ResultType<()> {
    builder.error_code.set_fn(|_, _| {
        Ok(ValueType::Enum(EnumType::new(
            None,
            [
                "access",
                "would-block",
                "already",
                "bad-descriptor",
                "busy",
                "deadlock",
                "quota",
                "exist",
                "file-too-large",
                "illegal-byte-sequence",
                "in-progress",
                "interrupted",
                "invalid",
                "io",
                "is-directory",
                "loop",
                "too-many-links",
                "message-size",
                "name-too-long",
                "no-device",
                "no-entry",
                "no-lock",
                "insufficient-memory",
                "insufficient-space",
                "not-directory",
                "not-empty",
                "not-recoverable",
                "unsupported",
                "no-tty",
                "no-such-device",
                "overflow",
                "not-permitted",
                "pipe",
                "read-only",
                "invalid-seek",
                "text-file-busy",
                "cross-device",
            ],
        )?))
    })?;

    builder.descriptor_flags.set_fn(|_, _| {
        Ok(ValueType::Flags(FlagsType::new(
            None,
            [
                "read",
                "write",
                "file-integrity-sync",
                "data-integrity-sync",
                "requested-write-sync",
                "mutate-directory",
            ],
        )?))
    })?;

    builder.descriptor_type.set_fn(|_, _| {
        Ok(ValueType::Enum(EnumType::new(
            None,
            [
                "unknown",
                "block-device",
                "character-device",
                "directory",
                "fifo",
                "symbolic-link",
                "regular-file",
                "socket",
            ],
        )?))
    })?;

    builder.descriptor_stat.set_fn(|builder, resources| {
        let descriptor_type_type =
            builder.get_type(resources, "descriptor_type", |b| &mut b.descriptor_type)?;
        let datetime_type = builder.get_type(resources, "datetime_type", |b| &mut b.datetime)?;

        Ok(ValueType::Record(RecordType::new(
            None,
            [
                ("type", descriptor_type_type),
                ("link-count", ValueType::U64),
                ("size", ValueType::U64),
                (
                    "data-access-timestamp",
                    ValueType::Option(OptionType::new(datetime_type.clone())),
                ),
                (
                    "data-modification-timestamp",
                    ValueType::Option(OptionType::new(datetime_type.clone())),
                ),
                (
                    "status-change-timestamp",
                    ValueType::Option(OptionType::new(datetime_type.clone())),
                ),
            ],
        )?))
    })?;

    builder
        .path_flags
        .set_fn(|_, _| Ok(ValueType::Flags(FlagsType::new(None, ["symlink-follow"])?)))?;

    builder.metadata_hash_value.set_fn(|_, _| {
        Ok(ValueType::Record(RecordType::new(
            None,
            [("lower", ValueType::U64), ("upper", ValueType::U64)],
        )?))
    })?;

    Ok(())
}

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    let instance = params
        .linker
        .instance_mut(&"wasi:filesystem/types@0.2.6".try_into()?)
        .expect("TODO: get filesystem instance");

    let descriptor_resource = &params.resources.filesystem.types.descriptor;

    instance.define_func(
        "[method]descriptor.read-via-stream",
        Func::new(
            &mut params.store,
            FuncType::new(
                [
                    ValueType::Borrow(descriptor_resource.clone()),
                    ValueType::U64,
                ],
                [ValueType::Result(waclay::ResultType::new(
                    Some(ValueType::Own(
                        params.resources.io.streams.input_stream.clone(),
                    )),
                    Some(params.types.error_code.clone().clone()),
                ))],
            ),
            |_, _, _| {
                println!("[STUB] [method]descriptor.read-via-stream called");
                Ok(())
            },
        ),
    )?;

    instance.define_func(
        "[method]descriptor.write-via-stream",
        Func::new(
            &mut params.store,
            FuncType::new(
                [
                    ValueType::Borrow(descriptor_resource.clone()),
                    ValueType::U64,
                ],
                [ValueType::Result(waclay::ResultType::new(
                    Some(ValueType::Own(
                        params.resources.io.streams.output_stream.clone(),
                    )),
                    Some(params.types.error_code.clone().clone()),
                ))],
            ),
            |_, _, _| {
                println!("[STUB] [method]descriptor.write-via-stream called");
                Ok(())
            },
        ),
    )?;

    instance.define_func(
        "[method]descriptor.append-via-stream",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::Borrow(descriptor_resource.clone())],
                [ValueType::Result(waclay::ResultType::new(
                    Some(ValueType::Own(
                        params.resources.io.streams.output_stream.clone(),
                    )),
                    Some(params.types.error_code.clone().clone()),
                ))],
            ),
            |_, _, _| {
                println!("[STUB] [method]descriptor.write-via-stream called");
                Ok(())
            },
        ),
    )?;

    instance.define_func(
        "[method]descriptor.get-flags",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::Borrow(descriptor_resource.clone())],
                [ValueType::Result(waclay::ResultType::new(
                    Some(params.types.descriptor_flags.clone().clone()),
                    Some(params.types.error_code.clone().clone()),
                ))],
            ),
            |_, _, _| {
                println!("[STUB] [method]descriptor.get-flags called");
                Ok(())
            },
        ),
    )?;

    instance.define_func(
        "[method]descriptor.stat",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::Borrow(descriptor_resource.clone())],
                [ValueType::Result(waclay::ResultType::new(
                    Some(params.types.descriptor_stat.clone().clone()),
                    Some(params.types.error_code.clone().clone()),
                ))],
            ),
            |_, _, _| {
                println!("[STUB] [method]descriptor.stat called");
                Ok(())
            },
        ),
    )?;

    instance.define_func(
        "[method]descriptor.stat-at",
        Func::new(
            &mut params.store,
            FuncType::new(
                [
                    ValueType::Borrow(descriptor_resource.clone()),
                    params.types.path_flags.clone().clone(),
                    ValueType::String,
                ],
                [ValueType::Result(waclay::ResultType::new(
                    Some(params.types.descriptor_stat.clone().clone()),
                    Some(params.types.error_code.clone().clone()),
                ))],
            ),
            |_, _, _| {
                println!("[STUB] [method]descriptor.stat-at called");
                Ok(())
            },
        ),
    )?;

    instance.define_func(
        "[method]descriptor.unlink-file-at",
        Func::new(
            &mut params.store,
            FuncType::new(
                [
                    ValueType::Borrow(descriptor_resource.clone()),
                    ValueType::String,
                ],
                [ValueType::Result(waclay::ResultType::new(
                    None,
                    Some(params.types.error_code.clone().clone()),
                ))],
            ),
            |_, _, _| {
                println!("[STUB] [method]descriptor.stat-at called");
                Ok(())
            },
        ),
    )?;

    instance.define_func(
        "[method]descriptor.metadata-hash",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::Borrow(descriptor_resource.clone())],
                [ValueType::Result(waclay::ResultType::new(
                    Some(params.types.metadata_hash_value.clone().clone()),
                    Some(params.types.error_code.clone().clone()),
                ))],
            ),
            |_, _, _| {
                println!("[STUB] [method]descriptor.metadata-hash called");
                Ok(())
            },
        ),
    )?;

    instance.define_func(
        "[method]descriptor.metadata-hash-at",
        Func::new(
            &mut params.store,
            FuncType::new(
                [
                    ValueType::Borrow(descriptor_resource.clone()),
                    params.types.path_flags.clone().clone(),
                    ValueType::String,
                ],
                [ValueType::Result(waclay::ResultType::new(
                    Some(params.types.metadata_hash_value.clone().clone()),
                    Some(params.types.error_code.clone().clone()),
                ))],
            ),
            |_, _, _| {
                println!("[STUB] [method]descriptor.metadata-hash-at called");
                Ok(())
            },
        ),
    )?;

    Ok(())
}
