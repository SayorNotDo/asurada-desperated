mod kernel_mapper;

pub use kernel_mapper::KernelMapper;

use core::num::NonZeroUsize;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Frame {
    physaddr: NonZeroUsize,
}
