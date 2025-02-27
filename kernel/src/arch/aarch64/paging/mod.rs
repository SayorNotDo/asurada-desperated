pub use super::CurrentArch;

pub type PageMapper = crate::allocator::mm::PageMapper<CurrentArch, crate::memory::TheFrameAllocator>;