use crate::allocator::mm::arch::Arch;
use core::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct PageEntry<A> {
    data: usize,
    phantom: PhantomData<A>,
}

impl<A: Arch> PageEntry<A> {
    #[inline(always)]
    pub fn new(address: usize, flags: usize) -> Self {
        let data = (((address >> A::PAGE_SHIFT) & A::ENTRY_ADDRESS_MASK) << A::ENTRY_ADDRESS_SHIFT
            | flags);
        Self::from_data(data)
    }

    #[inline(always)]
    pub fn from_data(data: usize) -> Self {
        Self {
            data,
            phantom: PhantomData,
        }
    }
}
