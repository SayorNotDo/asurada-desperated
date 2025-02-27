#![allow(unused)]

// 单PML4规格定义（Page Map Level 4，x86_64架构中的最高级页表）
pub const PML4_SIZE: usize = 0x0000_0080_0000_0000;
pub const PML4_MASK: usize = 0x0000_ff80_0000_0000;

/// Offset of recursive paging (deprecated, but still reserved)
pub const RECURSIVE_PAGE_OFFSET: usize = (-(PML4_SIZE as isize)) as usize;

/// 内核偏移
pub const KERNEL_OFFSET: usize = RECURSIVE_PAGE_OFFSET- PML4_SIZE;

pub const KERNEL_HEAP_OFFSET: usize = KERNEL_OFFSET - PML4_SIZE;
pub const KERNEL_HEAP_SIZE: usize = 1 * 1024 * 1024; // 1 MB