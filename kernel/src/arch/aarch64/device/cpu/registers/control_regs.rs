#![allow(unused)]

//! Functions to read and write control registers
//! 大量使用内联汇编宏，用于在Rust代码中直接嵌入汇编指令

use core::arch::asm;

// bitflags!宏生成的结构体会为每个定义的标志常量自动生成方法
bitflags! {
    // 该寄存器用于在ARMv8架构中配置内存访问的属性
    pub struct MairEl1: u64 {
        const DEVICE_MEMORY = 0x00 << 16;   // 设备内存
        const NORMAL_UNCACHED_MEMORY = 0x44 << 8; //普通未缓存内存
        const NORMAL_WRITEBACK_MEMORY = 0xff;   //普通写回内存
    }
}

pub unsafe fn tpidr_el1() -> u64 {
    let ret: u64;
    asm!("mrs {}, tpidr_el1", out(reg) ret);
    ret
}
