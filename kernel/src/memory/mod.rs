mod kernel_mapper;

pub use kernel_mapper::KernelMapper;

use core::num::NonZeroUsize;
use crate::allocator::mm::{FrameAllocator, FrameCount, FrameUsage, PhysicalAddress};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Frame {

    physaddr: NonZeroUsize,
}

#[derive(Debug)]
pub struct TheFrameAllocator;

impl FrameAllocator for TheFrameAllocator {
    unsafe fn allocate(&mut self, count: FrameCount) -> Option<PhysicalAddress> {
        todo!()
    }

    unsafe fn free(&mut self, address: PhysicalAddress, count: FrameCount) {
        todo!()
    }

    unsafe fn usage(&self) -> FrameUsage {
        todo!()
    }
}