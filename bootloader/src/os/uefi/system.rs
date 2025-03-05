use super::{
    TableHeader,
    boot::BootServices,
    config::ConfigurationTable,
    runtime::RuntimeServices,
    text::{TextInput, TextOutput},
};
use crate::os::uefi::prelude::*;
use core::slice;

#[repr(C)]
pub struct SystemTable {
    pub Hdr: TableHeader,
    pub FirmwareVendor: *const u16,
    pub FirmwareRevision: u32,
    pub ConsoleInHandle: Handle,
    pub ConsoleIn: &'static mut TextInput,
    pub ConsoleOutHandle: Handle,
    pub ConsoleOut: &'static mut TextOutput,
    pub RuntimeServices: &'static mut RuntimeServices,
    pub BootServices: &'static mut BootServices,
    Entries: usize,
    ConfigurationTables: *const ConfigurationTable,

}

impl SystemTable {
    pub fn config_table(&self) -> &'static [ConfigurationTable] {
        unsafe { slice::from_raw_parts(self.ConfigurationTables, self.Entries) }
    }
}
