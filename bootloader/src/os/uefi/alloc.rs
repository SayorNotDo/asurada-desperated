use crate::os::uefi::memory::MemoryType;
use crate::os::uefi::prelude::*;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::{self, NonNull};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
static mut UEFI: Option<NonNull<SystemTable>> = None;

pub unsafe fn init(table: &'static mut SystemTable) {
    UEFI = NonNull::new(table);
}

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let uefi = UEFI.expect("__rust_allocate: UEFI not initialized");
        let mut ptr = 0;
        let res = (uefi.as_ref().BootServices.AllocatePool)(
            MemoryType::EfiLoaderData,
            layout.size(),
            &mut ptr,
        );
        match res {
            Status::SUCCESS => ptr as *mut u8,
            _ => ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let uefi = UEFI.expect("__rust_deallocate: UEFI not initialized");
        let _ = (uefi.as_ref().BootServices.FreePool)(ptr as usize);
    }
}
