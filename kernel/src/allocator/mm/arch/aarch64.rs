use crate::allocator::mm::arch::Arch;
use crate::allocator::mm::{MemoryArea, PhysicalAddress, TableKind, VirtualAddress};
use core::arch::asm;

#[derive(Clone, Copy)]
pub struct AArch64Arch;

impl Arch for AArch64Arch {
    const PAGE_SHIFT: usize = 12;
    const ENTRY_ADDRESS_SHIFT: usize = 9;
    const PAGE_LEVELS: usize = 4;

    const ENTRY_ADDRESS_WIDTH: usize = 40;
    const ENTRY_FLAG_DEFAULT_PAGE: usize = Self::ENTRY_FLAG_PRESENT
        | 1 << 1    // Page flag
        | 1 << 10   // Access flag
        | Self::ENTRY_FLAG_NO_GLOBAL;
    const ENTRY_FLAG_DEFAULT_TABLE: usize =
        Self::ENTRY_FLAG_PRESENT | Self::ENTRY_FLAG_READWRITE | 1 << 1 | 1 << 10;
    const ENTRY_FLAG_PRESENT: usize = 1 << 0;
    const ENTRY_FLAG_READONLY: usize = 1 << 7;
    const ENTRY_FLAG_READWRITE: usize = 0;
    const ENTRY_FLAG_PAGE_USER: usize = 1 << 6;
    const ENTRY_FLAG_NO_EXEC: usize = 0b11 << 53;
    const ENTRY_FLAG_EXEC: usize = 0;
    const ENTRY_FLAG_GLOBAL: usize = 0;
    const ENTRY_FLAG_NO_GLOBAL: usize = 0;
    const ENTRY_FLAG_WRITE_COMBINING: usize = 0;

    const PHYS_OFFSET: usize = 0xFFFF_8000_0000_0000;

    unsafe fn init() -> &'static [MemoryArea] {
        unimplemented!("AArch64Arch::init unimplemented");
    }

    #[inline(always)]
    unsafe fn invalidate(address: VirtualAddress) {
        asm!("
            dsb ishst
            tlbi vaae1is, {}
            dsb ish
            isb
        ", in(reg) (address.data() >> Self::PAGE_SHIFT));
    }

    #[inline(always)]
    unsafe fn invalidate_all() {
        asm!(
            "
            dsb ishst
            tlbi vamlle1is
            dsb ish
            isb
        "
        );
    }

    #[inline(always)]
    unsafe fn table(table_kind: TableKind) -> PhysicalAddress {
        let address: usize;
        match table_kind {
            TableKind::User => {
                asm!("mrs {0}, ttbr0_el1", out(reg) address);
            }
            TableKind::Kernel => {
                asm!("mrs {0}, ttbr1_el1", out(reg) address);
            }
        }
        PhysicalAddress::new(address)
    }

    #[inline(always)]
    unsafe fn set_table(table_kind: TableKind, address: PhysicalAddress) {
        match table_kind {
            TableKind::User => {
                asm!("mrs {0}, ttbr0_el1", out(reg) address.data());
            }
            TableKind::Kernel => {
                asm!("mrs {0}, ttbr1_el1", out(reg) address.data());
            }
        }
        Self::invalidate_all();
    }

    fn virt_is_valid(_address: VirtualAddress) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {}
