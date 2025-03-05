#![no_std]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(internal_features)]
#![feature(let_chains)]
#![feature(lang_items)]
#![feature(int_roundings)]
#![feature(alloc_error_handler)]
#![cfg_attr(target_os = "uefi", no_main, feature(try_trait_v2))]

extern crate alloc;

#[macro_use]
mod os;
mod logger;

fn main() {
    println!("Bootloader starting {}", env!("CARGO_PKG_VERSION"));
}
