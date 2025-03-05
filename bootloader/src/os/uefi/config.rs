use super::guid::Guid;

#[repr(C)]
pub struct ConfigurationTable {
    pub VendorGrid: Guid,
    pub VendorTable: usize,
}