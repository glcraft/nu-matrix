pub mod timeout;

pub use std::sync::Arc;
pub use tokio::sync::Mutex;

pub use timeout::timeout;

pub type Instance<T> = Arc<Mutex<T>>;

#[inline]
pub fn new_instance<T>(value: T) -> Instance<T> {
    Arc::new(Mutex::new(value))
}

