use core::marker::PhantomData;

pub struct PageFlags<A> {
    data: usize,
    arch: PhantomData<A>
}