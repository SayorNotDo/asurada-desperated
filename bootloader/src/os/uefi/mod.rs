use crate::os::uefi::{
    reset::ResetType,
    status::{Result, Status},
};
use core::ptr;
use system::SystemTable;

#[macro_use]
mod macros;
mod boot;
mod config;
mod guid;
pub mod io;
mod runtime;
mod status;
mod system;
mod text;
mod time;

// 运行时支持
pub mod alloc;
mod memory;
mod prelude;
mod reset;
pub mod rt;
mod arch;

static mut HANDLE: Handle = Handle(0);
static mut SYSTEM_TABLE: *mut SystemTable = 0 as *mut SystemTable;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Event(pub usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Handle(pub usize);

#[repr(C)]
pub struct TableHeader {
    Signature: u64,
    Revision: u32,
    HeaderSize: u32,
    CRC32: u32,
    Reserved: u32,
}

pub fn handle() -> Handle {
    unsafe { HANDLE }
}

pub fn system_table() -> &'static SystemTable {
    unsafe { &*SYSTEM_TABLE }
}

pub unsafe fn system_table_mut() -> &'static mut SystemTable {
    unsafe { &mut *SYSTEM_TABLE }
}

fn status_to_result(status: Status) -> Result<usize> {
    match status {
        Status(ok) if status.is_success() => Ok(ok),
        err => Err(err),
    }
}

fn set_max_mode(output: &text::TextOutput) -> Result<()> {
    let mut max_i = None;
    let mut max_w = 0;
    let mut max_h = 0;

    for i in 0..output.Mode.MaxMode as usize {
        let mut w = 0;
        let mut h = 0;
        if (output.QueryMode)(output, i, &mut w, &mut h).is_success() {
            max_i = Some(i);
            max_w = w;
            max_h = h;
        }
    }

    if let Some(i) = max_i {
        status_to_result((output.SetMode)(output, i))?;
    }

    Ok(())
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> Status {
    let uefi = system_table();

    let _ = (uefi.BootServices.SetWatchdogTimer)(0, 0, 0, ptr::null());

    if let Err(err) = set_max_mode(uefi.ConsoleOut) {
        println!("Failed to set max mode: {}", err);
    }

    if let Err(err) = arch::main() {
        panic!("App error: {:?}", err);
    }
    (uefi.RuntimeServices.ResetSystem)(ResetType::Cold, Status(0), 0, ptr::null());
}
