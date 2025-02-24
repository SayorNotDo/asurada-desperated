use core::sync::atomic::AtomicU32;

/// A unique number used internally by kernel to identify CPUs.

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogicalCpuId(u32);

impl LogicalCpuId {
    // Bootstrap Processor
    pub const BSP: AtomicU32 = AtomicU32::new(0);

    pub const fn new(inner: u32) -> Self {
        Self(inner)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

impl core::fmt::Debug for LogicalCpuId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[logical cpu #{}]", self.0)
    }
}

impl core::fmt::Display for LogicalCpuId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{}", self.0)
    }
}
