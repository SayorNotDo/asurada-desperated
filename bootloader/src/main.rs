#![no_std]
#![feature(alloc_error_handler)]
#![cfg_attr(target_os = "uefi", no_main, feature(try_trait_v2))]

#[macro_use]
mod os;

extern crate alloc;

#[cfg(target_os = "uefi")]
#[macro_use]
extern crate uefi_std as uefi;

fn main() {

}
