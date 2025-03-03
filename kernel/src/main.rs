#![deny(unused_must_use)]
#![feature(allocator_api)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate bitflags;

use core::sync::atomic::{AtomicU32, Ordering};

use crate::consts::*;

#[macro_use]
#[allow(dead_code)]
mod arch;
use crate::arch::*;

/// 堆分配
mod allocator;

/// Memory management
mod memory;
mod panic;

mod cpu_set;
mod percpu;

/// 独立架构设备
mod devices;

mod elf;
mod log;

/// 全局内存分配器
#[cfg_attr(not(test), global_allocator)]
static ALLOCATOR: allocator::Allocator = allocator::Allocator;

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

macro_rules! linker_offset {
    ($($name:ident),*) => {$(
    #[inline]
    pub fn $name() -> usize {
        extern "C" {
            static $name: u8;
        }
        unsafe { &$name as *const u8 as usize }
    }
    )*
    };
}

mod kernel_executable_offsets {
    linker_offset!(
        __text_start,
        __text_end,
        __rodata_start,
        __rodata_end,
        __data_start,
        __data_end,
        __bss_start,
        __bss_end,
        __usercopy_start,
        __usercopy_end
    );
}
