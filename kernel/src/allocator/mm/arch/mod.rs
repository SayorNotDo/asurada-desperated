use crate::allocator::mm::{MemoryArea, PhysicalAddress, TableKind, VirtualAddress};
use core::ptr;

pub use self::aarch64::AArch64Arch;

#[cfg(target_pointer_width = "64")]
mod aarch64;

pub trait Arch: Clone + Copy {
    const PAGE_SHIFT: usize;
    const PAGE_ENTRY_SHIFT: usize;
    const PAGE_LEVELS: usize;

    const ENTRY_ADDRESS_WIDTH: usize; // 页表项中物理内存地址的位数
    const ENTRY_ADDRESS_SHIFT: usize = Self::PAGE_SHIFT; // 页表项中物理内存地址的偏移
    const ENTRY_FLAG_DEFAULT_PAGE: usize;
    const ENTRY_FLAG_DEFAULT_TABLE: usize;
    const ENTRY_FLAG_PRESENT: usize;
    const ENTRY_FLAG_READONLY: usize;
    const ENTRY_FLAG_READWRITE: usize;
    const ENTRY_FLAG_PAGE_USER: usize; // 用户页标志
    const ENTRY_FLAG_TABLE_USER: usize = Self::ENTRY_FLAG_PAGE_USER;
    const ENTRY_FLAG_NO_EXEC: usize;
    const ENTRY_FLAG_EXEC: usize;
    const ENTRY_FLAG_GLOBAL: usize;
    const ENTRY_FLAG_NO_GLOBAL: usize;
    const ENTRY_FLAG_WRITE_COMBINING: usize;

    const PHYS_OFFSET: usize;

    const PAGE_SIZE: usize = 1 << Self::PAGE_SHIFT;
    const PAGE_OFFSET_MASK: usize = Self::PAGE_SIZE - 1;
    const PAGE_ADDRESS_SHIFT: usize =
        Self::PAGE_LEVELS * Self::PAGE_ENTRY_SHIFT + Self::PAGE_ADDRESS_SHIFT;
    const PAGE_ADDRESS_SIZE: u64 = 1 << (Self::PAGE_ADDRESS_SHIFT as u64);
    const PAGE_ADDRESS_MASK: usize = (Self::PAGE_ADDRESS_SIZE - (Self::PAGE_SIZE as u64)) as usize;
    const PAGE_ENTRY_SIZE: usize = 1 << (Self::PAGE_SHIFT - Self::PAGE_ENTRY_SHIFT);
    const PAGE_ENTRIES: usize = 1 << Self::PAGE_ENTRY_SHIFT;
    const PAGE_ENTRY_MASK: usize = Self::PAGE_ENTRIES - 1;
    const PAGE_NEGATIVE_MASK: usize = !(Self::PAGE_ADDRESS_SIZE - 1) as usize;

    const ENTRY_ADDRESS_SIZE: usize = 1 << Self::ENTRY_ADDRESS_WIDTH;
    const ENTRY_ADDRESS_MASK: usize = !(Self::PAGE_ADDRESS_SIZE - 1) as usize;
    const ENTRY_FLAGS_MASK: usize = !(Self::ENTRY_ADDRESS_MASK << Self::ENTRY_ADDRESS_SHIFT);

    unsafe fn init() -> &'static [MemoryArea];

    #[inline(always)]
    unsafe fn read<T>(address: VirtualAddress) -> T {
        ptr::read(address.data() as *const T)
    }

    #[inline(always)]
    unsafe fn write<T>(address: VirtualAddress, value: T) {
        ptr::write(address.data() as *mut T, value)
    }

    #[inline(always)]
    unsafe fn write_bytes(address: VirtualAddress, value: u8, count: usize) {
        ptr::write_bytes(address.data() as *mut u8, value, count);
    }

    unsafe fn invalidate(address: VirtualAddress);

    #[inline(always)]
    unsafe fn invalidate_all() {
        Self::set_table(TableKind::User, Self::table(TableKind::User));
    }

    unsafe fn table(table_kind: TableKind) -> PhysicalAddress;

    unsafe fn set_table(table_kind: TableKind, address: PhysicalAddress);

    #[inline(always)]
    unsafe fn phys_to_virt(phys: PhysicalAddress) -> VirtualAddress {
        match phys.data().checked_add(Self::PHYS_OFFSET) {
            Some(some) => VirtualAddress::new(some),
            None => panic!("phys_to_virt({:#x}) overflow", phys.data()),
        }
    }

    fn virt_is_valid(address: VirtualAddress) -> bool;
}
