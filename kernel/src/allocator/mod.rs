use crate::memory::KernelMapper;

#[cfg(not(feature = "slab"))]
pub use self::linked_list::Allocator;

#[cfg(not(feature = "slab"))]
mod linked_list;

#[cfg(feature = "slab")]
mod slab;
pub mod mm;

unsafe fn map_heap(mapper: &mut KernelMapper, offset: usize, size: usize) {}
