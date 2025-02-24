//! 中断指令

use core::arch::asm;

pub mod trace;

/// 禁用中断
#[inline(always)]
pub unsafe fn disable() {
    asm!("msr daifset, #2");
}

/// 启用中断并等待中断的发生
/// wfi 指令会使处理器进入低功耗模式，直到有中断发生
/// 属于原子操作的方式，意味其确保启用中断和等待中断这两个操作是原子执行的
#[inline(always)]
pub unsafe fn enable_and_halt() {
    asm!("msr daifclr, #2");
    asm!("wfi");
}

/// 启用中断并执行无操作
/// msr daifclr, #2 指令清除IRQ的标志，允许中断触发
/// nop 指令使处理器空闲一个周期（处理器不会进入低功耗模式），等待下一个中断
#[inline(always)]
pub unsafe fn enable_and_nop() {
    asm!("msr daifclr, #2");
    asm!("nop");
}

/// 中断等待
#[inline(always)]
pub unsafe fn halt() {
    asm!("wfi");
}

/// 暂停指令
/// 通过执行一个 nop 指令来暂停执行，常用于优化，比如防止循环忙等待占用过多的CPU资源，标记为安全（无任何副作用）
pub fn pause() {
    unsafe {
        asm!("nop");
    }
}
