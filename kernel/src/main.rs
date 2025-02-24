#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[macro_use]
extern crate bitflags;

use core::sync::atomic::{AtomicU32, Ordering};
#[macro_use]
#[allow(dead_code)]
mod arch;
use crate::arch::*;

/// Memory management
mod memory;
mod panic;

mod cpu_set;
mod percpu;

mod log;

/// Get the current CPU's scheduling ID
fn cpu_id() -> crate::cpu_set::LogicalCpuId {
    crate::percpu::PercpuBlock::current().cpu_id
}

static CPU_COUNT: AtomicU32 = AtomicU32::new(0);

/// Get the number of CPUs currently active
#[inline(always)]
fn cpu_count() -> u32 {
    CPU_COUNT.load(Ordering::Relaxed)
}

struct Bootstrap {
    base: crate::memory::Frame,
    page_count: usize,
    env: &'static [u8],
}

static BOOTSTRAP: spin::Once<Bootstrap> = spin::Once::new();

#[no_mangle] // 不重整函数名
pub extern "C" fn _start() {}
