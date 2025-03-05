#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct PhysicalAddress(pub u64);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct VirtualAddress(pub u64);

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct MemoryDescriptor {
    pub Type: u32,
    pub PhysicalStart: PhysicalAddress,
    pub VirtualStart: VirtualAddress,
    pub NumberOfPages: u64,
    pub Attributes: u64,
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum MemoryType {
    EfiReservedMemoryType,
    EfiLoaderData,
}