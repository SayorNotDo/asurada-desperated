use super::TableHeader;
use super::status::Status;
use super::time::{Time, TimeCapabilities};
use crate::os::uefi::reset::ResetType;

#[repr(C)]
pub struct RuntimeServices {
    pub Hdr: TableHeader,
    pub GetTime: extern "efiapi" fn(Time: &mut Time, Capabilities: *mut TimeCapabilities) -> Status,
    pub ResetSystem: extern "efiapi" fn(
        ResetType: ResetType,
        ResetStatus: Status,
        DataSize: usize,
        ResetData: *const u8,
    ) -> !,
}
