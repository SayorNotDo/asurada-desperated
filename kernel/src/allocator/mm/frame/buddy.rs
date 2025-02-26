use core::marker::PhantomData;

use super::PhysicalAddress;

#[repr(transparent)]
struct BuddyUsage(u8);

#[repr(C, packed)]
struct BuddyEntry<A> {
    base: PhysicalAddress,
    size: usize,
    skip: usize,
    used: usize,
    phantom: PhantomData<A>,
}

impl<A> Clone for BuddyEntry<A> {
    fn clone(&self) -> Self {
        Self {
            base: self.base,
            size: self.size,
            skip: self.skip,
            used: self.used,
            phantom: PhantomData,
        }
    }
}

