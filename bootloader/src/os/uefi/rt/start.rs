use crate::os::uefi::prelude::*;

#[unsafe(no_mangle)]
pub unsafe extern "efiapi" fn efi_main(
    handle: Handle,
    system_table: &'static mut SystemTable,
) -> Status {
    // unsafe {
    //     extern "C" {
    //         fn main() -> Status;
    //     }
    //     crate::os::uefi::HANDLE = handle;
    //     crate::os::uefi::SYSTEM_TABLE = system_table;
    //
    //     crate::os::uefi::alloc::init(::core::mem::transmute(system_table));
    //
    //     // main()
    // }
    Status::SUCCESS
}
