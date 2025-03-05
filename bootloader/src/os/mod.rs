#[cfg(target_os = "uefi")]
#[allow(unused_imports)]
pub use self::uefi::*;

#[cfg(target_os = "uefi")]
#[macro_use]
mod uefi;
