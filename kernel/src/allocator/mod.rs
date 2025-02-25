use crate::memory::KernelMapper;

#[cfg(not(feature = "slab"))]
mod linked_list;

#[cfg(feature = "slab")]
mod slab;

unsafe fn map_heap(mapper: &mut KernelMapper, offset: usize, size: usize) {}
