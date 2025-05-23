use redoxfs::Disk;

#[cfg(all(target_arch = "x86", target_os = "none"))]
pub use self::bios::*;

#[cfg(all(target_arch = "x86", target_os = "none"))]
#[macro_use]
mod bios;

#[cfg(target_os = "uefi")]
#[allow(unused_imports)]
pub use self::uefi::*;

#[cfg(target_os = "uefi")]
#[macro_use]
mod uefi;

/// 描述EFI启动时系统的硬件描述方式，基于不同的架构（x86/ARM）处理不同的硬件描述方案
#[derive(Clone, Copy, Debug)]
pub enum OsHwDesc {
    Acpi(u64, u64),         // 高级配置和电源接口：（根系统描述指针起始地址，数据结构大小）
    DeviceTree(u64, u64),   // 存储设备树所在的物理地址信息：（起始地址，数据结构的大小）
    NotFound,               // 用于错误处理或表示未知硬件描述情况
}

#[derive(Clone, Copy, Debug)]
pub enum OsKey {
    Left,
    Right,
    Up,
    Down,
    Backspace,
    Delete,
    Enter,
    Char(char),
    Other,
}

// Keep synced with BootloaderMemoryKind in kernel
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u64)]
pub enum OsMemoryKind {
    Null = 0,
    Free = 1,
    Reclaim = 2,
    Reserved = 3,
}

// Keep synced with BootloaderMemoryEntry in kernel
#[derive(Clone, Copy, Debug)]
#[repr(C, packed(8))]
pub struct OsMemoryEntry {
    pub base: u64,
    pub size: u64,
    pub kind: OsMemoryKind,
}

#[derive(Clone, Copy, Debug)]
pub struct OsVideoMode {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub base: u64,
}

pub trait Os<D: Disk, V: Iterator<Item = OsVideoMode>> {
    fn name(&self) -> &str;

    fn alloc_zeroed_page_aligned(&self, size: usize) -> *mut u8;

    #[allow(dead_code)]
    fn page_size(&self) -> usize;

    fn filesystem(&self, password_opt: Option<&[u8]>) -> syscall::Result<redoxfs::FileSystem<D>>;

    fn hwdesc(&self) -> OsHwDesc;

    fn video_outputs(&self) -> usize;
    fn video_modes(&self, output_i: usize) -> V;
    fn set_video_mode(&self, output_i: usize, mode: &mut OsVideoMode);
    fn best_resolution(&self, output_i: usize) -> Option<(u32, u32)>;

    fn get_key(&self) -> OsKey;

    fn clear_text(&self);
    fn get_text_position(&self) -> (usize, usize);
    fn set_text_position(&self, x: usize, y: usize);
    fn set_text_highlight(&self, highlight: bool);
}
