pub use std::process::id;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::parent_id;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::parent_id;
