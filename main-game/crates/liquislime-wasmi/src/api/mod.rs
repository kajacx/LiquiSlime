mod convert_helpers;
mod from_game_api;
mod linker;
mod to_game_api;

pub use from_game_api::*;
pub use linker::*;
pub use to_game_api::*;

pub trait NativeWasmType {}

impl NativeWasmType for i32 {}
impl NativeWasmType for i64 {}

impl NativeWasmType for u32 {}
impl NativeWasmType for u64 {}

impl NativeWasmType for f32 {}
impl NativeWasmType for f64 {}
