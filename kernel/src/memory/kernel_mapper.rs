pub struct KernelMapper {
    mapper: crate::paging::PageMapper,
    ro: bool,
}
