// Common modules

mod process;
mod worker;

pub use worker::{ProcessConfig, Worker};

// OS-specific modules

#[cfg_attr(target_os = "linux", path = "linux/mod.rs")]
#[cfg_attr(target_os = "windows", path = "windows/mod.rs")]
mod os;

#[cfg(target_os = "windows")]
#[macro_use]
extern crate lazy_static;

// Re-exported functions from sub-crates
pub use iris_policy::{
    downcast_to_handle, set_unmanaged_handle_inheritable, CrossPlatformHandle, Handle, Policy,
};
