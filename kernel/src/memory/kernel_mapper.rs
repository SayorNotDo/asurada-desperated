use core::sync::atomic;
use crate::cpu_set::LogicalCpuId;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::allocator::mm::{PageMapper, TableKind};

const NO_PROCESSOR: usize = !0;
static LOCK_OWNER: AtomicUsize = AtomicUsize::new(NO_PROCESSOR);
static LOCK_COUNT: AtomicUsize = AtomicUsize::new(0);

pub struct KernelMapper {
    mapper: crate::paging::PageMapper,
    ro: bool,
}

impl KernelMapper {
    fn lock_inner(current_processor: usize) -> bool {
        loop {
            match LOCK_OWNER.compare_exchange_weak(
                NO_PROCESSOR,
                current_processor,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(id) if id == current_processor => break,
                Err(_) => core::hint::spin_loop()
            }
        }
        let prev_count = LOCK_COUNT.fetch_add(1, Ordering::Relaxed);
        atomic::compiler_fence(Ordering::Acquire);

        prev_count > 0
    }

    pub unsafe fn lock_for_manual_mapper(
        current_processor: LogicalCpuId,
        mapper: crate::paging::PageMapper,
    ) -> Self {
        let ro = Self::lock_inner(current_processor.get() as usize);
        Self { mapper, ro }
    }
    pub fn lock_manually(current_processor: LogicalCpuId) -> Self {
        unsafe {
            Self::lock_for_manual_mapper(
                current_processor, PageMapper::current(TableKind::Kernel, crate::memory::TheFrameAllocator)
            )
        }
    }

    pub fn lock() -> Self {
        Self::lock_manually(crate::cpu_id())
    }
}
