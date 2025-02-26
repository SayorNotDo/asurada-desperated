use core::marker::PhantomData;
use crate::allocator::mm::{VirtualAddress, Arch};

#[must_use = "The page table must be flushed, or the changes unsafely ignored"]
pub struct PageFlush<A> {
    virt: VirtualAddress,
    phantom: PhantomData<A>
}

impl <A: Arch> PageFlush<A> {
    pub fn new(virt: VirtualAddress) -> Self {
        Self {
            virt,
            phantom: PhantomData
        }
    }
}