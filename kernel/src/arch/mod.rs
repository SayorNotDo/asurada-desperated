#[cfg(target_arch = "aarch64.rs")]
#[macro_use]
pub mod aarch64;
#[cfg(target_arch = "aarch64.rs")]
pub use self::aarch64::*;
