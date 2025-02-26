use crate::allocator::mm::arch::Arch;
use crate::allocator::mm::{PhysicalAddress, VirtualAddress};
use core::marker::PhantomData;
use super::PageEntry;

pub struct PageTable<A> {
    base: VirtualAddress,
    phys: PhysicalAddress,
    level: usize,
    phantom: PhantomData<A>,
}

impl<A: Arch> PageTable<A> {
    pub unsafe fn new(base: VirtualAddress, phys: PhysicalAddress, level: usize) -> Self {
        Self {
            base,
            phys,
            level,
            phantom: PhantomData,
        }
    }

    pub unsafe fn entry_virt(&self, i: usize) -> Option<VirtualAddress> {
        if i < A::PAGE_ENTRIES {
            Some(self.virt().add(i * A::PAGE_ENTRY_SIZE))
        } else {
            None
        }
    }

    pub unsafe fn entry(&self, i: usize) -> Option<PageEntry<A>> {
        let addr = self.entry_virt(i)?;
        Some(PageEntry::from_data(A::read::<usize>(addr)))
    }

    pub unsafe fn set_entry(&mut self, i: usize, entry: PageEntry<A>) -> Option<()> {
        let addr = self.entry_virt(i)?;
        A::write::<usize>(addr, entry.data());
        Some(())
    }

    pub unsafe fn index_of(&self, address: VirtualAddress) -> Option<usize> {
        let address = VirtualAddress::new(address.data() & A::PAGE_ADDRESS_MASK);
        let level_shift = self.level * A::PAGE_ENTRY_SHIFT + A::PAGE_SHIFT;

        let level_mask = A::PAGE_ENTRIES
            .wrapping_shl(level_shift as u32)
            .wrapping_sub(1);
        if address >= self.base && address <= self.base.add(level_mask) {
            Some((address.data() >> level_shift) & A::PAGE_ENTRY_MASK)
        } else {
            None
        }
    }

    pub unsafe fn next(&self, i: usize) -> Option<Self> {
        if self.level == 0 {
            return None;
        }

        Some(PageTable::new(
            self.entry_base(i)?,
            self.entry(i)?.address().ok()?,
            self.level - 1,
        ))
    }
}
