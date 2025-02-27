use crate::allocator::mm::Arch;
use core::marker::PhantomData;


#[derive(Clone, Copy)]
pub struct PageFlags<A> {
    data: usize,
    arch: PhantomData<A>,
}

impl<A: Arch> PageFlags<A> {
    #[inline(always)]
    pub fn new() -> Self {
        unsafe {
            Self::from_data(
                A::ENTRY_FLAG_DEFAULT_PAGE
                    | A::ENTRY_FLAG_READONLY
                    | A::ENTRY_FLAG_NO_EXEC
                    | A::ENTRY_FLAG_NO_GLOBAL,
            )
        }
    }

    #[inline(always)]
    pub unsafe fn from_data(data: usize) -> Self {
        Self {
            data,
            arch: PhantomData,
        }
    }

    #[inline(always)]
    pub fn data(&self) -> usize {
        self.data
    }
}
