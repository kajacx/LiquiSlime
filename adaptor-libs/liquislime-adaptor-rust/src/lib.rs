mod convert;
mod game_api;
mod types;

pub use convert::*;
pub use game_api::*;
pub use try_lock::TryLock;
pub use types::*;

pub trait BehaviourAdaptor: Sized {
    fn new() -> Self;

    fn update(&mut self, delta_time: f64);
}

#[macro_export]
macro_rules! export_adaptor {
    ($adaptor:ty, $val:expr) => {
        static ADAPTOR: ::liquislime_adaptor_rust::TryLock<$adaptor> =
            ::liquislime_adaptor_rust::TryLock::new(
                // <$adaptor as ::liquislime_adaptor_rust::BehaviourAdaptor>::new(),
                $val,
            );

        #[unsafe(no_mangle)]
        pub extern "C" fn update(time_passed: f64) {
            ADAPTOR.try_lock().unwrap().update(time_passed as f64);
        }
    };
}
