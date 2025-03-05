use crate::os::uefi::TableHeader;
use crate::os::uefi::memory::MemoryType;
use crate::os::uefi::status::Status;

#[repr(C)]
pub struct BootServices {
    pub Hdr: TableHeader,
    pub AllocatePool:
        extern "efiapi" fn(PoolType: MemoryType, Size: usize, Buffer: &mut usize) -> Status,
    pub FreePool: extern "efiapi" fn(Buffer: usize) -> Status,
    pub SetWatchdogTimer: extern "efiapi" fn(
        Timeout: usize,
        WatchdogCode: u64,
        DataSize: usize,
        WatchdogData: *const u16,
    ) -> Status,
}
