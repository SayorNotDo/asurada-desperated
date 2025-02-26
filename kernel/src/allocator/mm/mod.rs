pub use self::frame::*;

mod frame;
mod arch;
mod page;

pub const KILOBYTE: usize = 1024;
pub const MEGABYTE: usize = KILOBYTE * 1024;
pub const GIGABYTE: usize = MEGABYTE * 1024;
#[cfg(target_pointer_width = "64")]
pub const TERABYTE: usize = GIGABYTE * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableKind {
    // 用户态页表
    User,
    // 内核态页表
    Kernel,
}

/// 物理内存地址，保持与无符号整型相同的内存布局
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
    #[inline(always)]
    pub const fn new(address: usize) -> Self {
        Self(address)
    }

    #[inline(always)]
    pub fn data(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn add(self, offset: usize) -> Self {
        Self(self.0 + offset)
    }
}

/// 虚拟内存地址
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VirtualAddress(usize);

impl VirtualAddress {
    #[inline(always)]
    pub const fn new(address: usize) -> Self {
        Self(address)
    }

    #[inline(always)]
    pub fn data(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn add(self, offset: usize) -> Self {
        Self(self.0 + offset)
    }

    #[inline(always)]
    pub fn kind(&self) -> TableKind {
        if (self.0 as isize) < 0 {
            TableKind::Kernel
        } else {
            TableKind::User
        }
    }
}

impl core::fmt::Debug for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[virt {:#0x}]", self.data())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MemoryArea {
    pub base: PhysicalAddress,
    pub size: usize,
}
