use crate::allocator::mm::arch::Arch;
use crate::allocator::mm::page::{PageTable, PageFlags, PageFlush};
use crate::allocator::mm::{FrameAllocator, PhysicalAddress, TableKind, VirtualAddress};
use core::marker::PhantomData;
use crate::allocator::mm::page::entry::PageEntry;

pub struct PageMapper<A, F> {
    table_kind: TableKind,
    table_addr: PhysicalAddress,
    allocator: F,
    _phantom: PhantomData<fn() -> A>, // 绑定架构类型
}

impl<A: Arch, F: FrameAllocator> PageMapper<A, F> {
    pub unsafe fn new(table_kind: TableKind, table_addr: PhysicalAddress, allocator: F) -> Self {
        Self {
            table_kind,
            table_addr,
            allocator,
            _phantom: PhantomData,
        }
    }

    pub unsafe fn create(table_kind: TableKind, mut allocator: F) -> Option<Self> {
        let table_addr = allocator.allocate_one()?;
        Some(Self::new(table_kind, table_addr, allocator))
    }

    pub unsafe fn current(table_kind: TableKind, allocator: F) -> Self {
        let table_addr = A::table(table_kind);
        Self::new(table_kind, table_addr, allocator)
    }

    pub fn is_current(&self) -> bool {
        unsafe { self.table().phys() == A::table(self.table_kind) }
    }

    pub unsafe fn make_current(&self) {
        A::set_table(self.table_kind, self.table_addr);
    }

    pub fn table(&self) -> PageTable<A> {
        // SAFETY: The only way to initialize a PageMapper is via new(), and we assume it upholds
        // all necessary invariants for this to be safe.
        unsafe { PageTable::new(VirtualAddress::new(0), self.table_addr, A::PAGE_LEVELS - 1) }
    }

    pub fn allocator(&self) -> &F {
        &self.allocator
    }

    pub fn allocator_mut(&mut self) -> &mut F {
        &mut self.allocator
    }

    // 重新映射虚拟地址到物理地址，允许修改页表条目中的物理地址和标志位
    pub unsafe fn remap_with_full(
        &mut self,
        virt: VirtualAddress,
        f: impl FnOnce(PhysicalAddress, PageFlags<A>) -> (PhysicalAddress, PageFlags<A>),
    ) -> Option<(PageFlags<A>, PhysicalAddress, PageFlush<A>)> {
        self.visit(virt, |p1, i| {
            let old_entry = p1.entry(i)?;
            let old_phys = old_entry.address().ok()?;
            let old_flags = old_entry.flags();

            let (new_phys, new_flags) = f(old_phys, old_flags);
            // TODO: Higher-level PageEntry::new interface?
            let new_entry = PageEntry::new(new_phys.data(), new_flags.data());
            p1.set_entry(i, new_entry);
            Some((old_flags, old_phys, PageFlush::new(virt)))
        }).flatten()
    }

    fn visit<T> (&self, virt: VirtualAddress, f: impl FnOnce(&mut PageTable<A>, usize) -> T) -> Option<T> {
        let mut table = self.table();
        unsafe {
            loop {
                let i = table.index_of(virt)?;
                if table.level() == 0 {
                    return Some(f(&mut table, i));
                } else {
                    table = table.next(i)?;
                }
            }
        }
    }
}
