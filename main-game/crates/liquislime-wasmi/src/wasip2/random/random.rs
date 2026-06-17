use crate::{component_adaptor::ResultType, wasip2::AddToLinkerParams};
use waclay::*;

pub fn add_to_linker(params: &mut AddToLinkerParams) -> ResultType<()> {
    let instance = params
        .linker
        .define_instance("wasi:random/random@0.2.6".try_into()?)?;

    instance.define_func(
        "get-random-bytes",
        Func::new(
            &mut params.store,
            FuncType::new(
                [ValueType::U64],
                [ValueType::List(ListType::new(ValueType::U8))],
            ),
            move |mut caller, params: &[Value], results: &mut [Value]| {
                println!("[STUB] get-random-bytes called");
                if let Value::U64(len) = params[0] {
                    println!("len: {len}");
                    let bytes = caller
                        .data_mut()
                        .rng
                        .by_ref()
                        .take(len as usize)
                        .map(Value::U8)
                        .collect::<Vec<_>>();
                    results[0] = Value::List(List::new(ListType::new(ValueType::U8), bytes)?);
                    Ok(())
                } else {
                    todo!()
                }
            },
        ),
    )?;

    Ok(())
}

pub struct LoopingRng {
    values: [u8; 16],
    index: usize,
}

impl LoopingRng {
    pub fn new() -> Self {
        Self {
            values: [
                169, 66, 136, 29, 214, 223, 192, 76, 87, 126, 192, 113, 243, 58, 123, 151,
            ],
            index: 15, // Increment before return is easier, so start at max index
        }
    }
}

impl Iterator for LoopingRng {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.index = (self.index + 1) % self.values.len();
        return Some(self.values[self.index]);
    }
}
