mod kernel_mapper;

use core::num::NonZeroUsize;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Frame {
    physaddr: NonZeroUsize,
}
