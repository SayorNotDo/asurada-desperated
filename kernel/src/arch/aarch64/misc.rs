use crate::percpu::PercpuBlock;

impl PercpuBlock {
    pub fn current() -> &'static Self {
        unsafe { &*(crate::device::cpu::registers::control_regs::tpidr_el1() as *const Self) }
    }
}
