use alloc::vec::Vec;
use core::{cell::RefCell, mem, ptr, slice};
use std::proto::Protocol;
use uefi::{
    boot::LocateSearchType,
    memory::MemoryType,
    reset::ResetType,
    status::{Result, Status},
    system::SystemTable,
    text::TextInputKey,
    Handle,
};

use crate::os::{Os, OsHwDesc, OsKey, OsVideoMode};

use self::{
    device::{device_path_to_string, disk_device_priority},
    disk::DiskEfi,
    display::{EdidActive, Output},
    video_mode::VideoModeIter,
};

mod acpi;
mod arch;
mod device;
mod disk;
mod display;
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
pub mod dtb;
mod memory_map;
mod video_mode;

#[cfg(target_arch = "riscv64")]
pub use arch::efi_get_boot_hartid;

pub(crate) fn page_size() -> usize {
    // EDK2 always uses 4096 as the page size
    4096
}

pub(crate) fn alloc_zeroed_page_aligned(size: usize) -> *mut u8 {
    assert!(size != 0);

    let page_size = page_size();
    let pages = (size + page_size - 1) / page_size;

    let ptr = {
        // Max address mapped by src/arch paging code (8 GiB)
        let mut ptr = 0x2_0000_0000;
        status_to_result((std::system_table().BootServices.AllocatePages)(
            1,                                  // AllocateMaxAddress
            MemoryType::EfiRuntimeServicesData, // Keeps this memory out of free space list
            pages,
            &mut ptr,
        ))
        .unwrap();
        ptr as *mut u8
    };

    assert!(!ptr.is_null());
    unsafe { ptr::write_bytes(ptr, 0, pages * page_size) };
    ptr
}

pub struct OsEfi {
    // EFI相关的结构体，存储EFI系统表的，其中包含引导服务和运行时服务
    st: &'static SystemTable,
    // 可变借用运行运行时动态修改输出设备的信息
    // Output 输出设备
    // Extended Display Identification Data 描述显示器的数据结构
    outputs: RefCell<Vec<(Output, Option<EdidActive>)>>,
}

impl OsEfi {
    pub fn new() -> Self {
        let st = std::system_table();
        let mut outputs = Vec::<(Output, Option<EdidActive>)>::new();
        {
            let guid = Output::guid();
            let mut handles = Vec::with_capacity(256);
            let mut len = handles.capacity() * size_of::<Handle>();
            match status_to_result((st.BootServices.LocateHandle)(
                LocateSearchType::ByProtocol,
                &guid,
                0,
                &mut len,
                handles.as_mut_ptr(),
            )) {
                Ok(_) => {
                    unsafe {
                        handles.set_len(len / size_of::<Handle>());
                    }
                    'handles: for handle in handles {
                        // TODO: do we have to query all modes to get good edid?
                        match Output::handle_protocol(handle) {
                            Ok(output) => {
                                log::debug!(
                                    "Output {:?} at {:x}",
                                    handle,
                                    output.0.Mode.FrameBufferBase
                                );

                                // 帧缓冲地址为 0 则跳过该输出设备（跳过无效设备）
                                if output.0.Mode.FrameBufferBase == 0 {
                                    log::debug!("Skipping output with frame buffer base of 0");
                                    continue 'handles;
                                }

                                // 帧缓冲地址匹配，防止重复的输出设备
                                for other_output in outputs.iter() {
                                    if output.0.Mode.FrameBufferBase
                                        == other_output.0 .0.Mode.FrameBufferBase
                                    {
                                        log::debug!("Skipping output with frame buffer base matching another output");
                                        continue 'handles;
                                    }
                                }

                                outputs.push((
                                    output,
                                    match EdidActive::handle_protocol(handle) {
                                        Ok(efi_edid) => Some(efi_edid),
                                        Err(err) => {
                                            log::warn!(
                                                "Failed to get EFI EDID from handle {:?}: {:?}",
                                                handle,
                                                err
                                            );
                                            None
                                        }
                                    },
                                ));
                            }
                            Err(err) => {
                                log::warn!(
                                    "Failed to get Output from handle {:?}: {:?}",
                                    handle,
                                    err
                                );
                            }
                        }
                    }
                }
                Err(err) => {
                    log::warn!("Failed to locate Outputs: {:?}", err);
                }
            }
        }
        Self {
            st,
            outputs: RefCell::new(outputs),
        }
    }
}

impl Os<DiskEfi, VideoModeIter> for OsEfi {
    #[cfg(target_arch = "aarch64")]
    fn name(&self) -> &str {
        "aarch64/UEFI"
    }

    #[cfg(target_arch = "x86_64")]
    fn name(&self) -> &str {
        "x86_64/UEFI"
    }

    #[cfg(target_arch = "riscv64")]
    fn name(&self) -> &str {
        "riscv64/UEFI"
    }

    fn alloc_zeroed_page_aligned(&self, size: usize) -> *mut u8 {
        alloc_zeroed_page_aligned(size)
    }

    fn page_size(&self) -> usize {
        page_size()
    }

    fn filesystem(
        &self,
        password_opt: Option<&[u8]>,
    ) -> syscall::Result<redoxfs::FileSystem<DiskEfi>> {
        // Search for RedoxFS on disks in prioritized order
        println!("Looking for RedoxFS:");
        for device in disk_device_priority() {
            println!(" - {}", device_path_to_string(device.device_path.0));

            if !device.disk.0.Media.MediaPresent {
                continue;
            }

            let block = if device.disk.0.Media.LogicalPartition {
                0
            } else {
                //TODO: get block from partition table
                2 * crate::MIBI as u64 / redoxfs::BLOCK_SIZE
            };

            match redoxfs::FileSystem::open(device.disk, password_opt, Some(block), false) {
                Ok(ok) => return Ok(ok),
                Err(err) => match err.errno {
                    // Ignore header not found error
                    syscall::ENOENT => (),
                    // Print any other errors
                    _ => {
                        log::warn!("BlockIo error: {:?}", err);
                    }
                },
            }
        }

        log::warn!("No RedoxFS partitions found");
        Err(syscall::Error::new(syscall::ENOENT))
    }

    fn hwdesc(&self) -> OsHwDesc {
        println!("Looking for Hardware Descriptor");
        //TODO: if both DTB and ACPI are found, we should probably let the OS choose what to use?

        // For now we will prefer DTB on platforms that have it
        #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
        if let Some((addr, size)) = dtb::find_dtb(self) {
            println!("Found DTB at {:x}, size: {}", addr, size);
            return OsHwDesc::DeviceTree(addr, size);
        }

        if let Some((addr, size)) = acpi::find_acpi_table_pointers(self) {
            return OsHwDesc::Acpi(addr, size);
        }

        OsHwDesc::NotFound
    }

    fn video_outputs(&self) -> usize {
        self.outputs.borrow().len()
    }

    fn video_modes(&self, output_i: usize) -> VideoModeIter {
        let output_opt = match self.outputs.borrow_mut().get_mut(output_i) {
            Some(output) => unsafe {
                // Hack to enable clone
                let ptr = output.0 .0 as *mut _;
                Some(Output::new(&mut *ptr))
            },
            None => None,
        };
        VideoModeIter::new(output_opt)
    }

    fn set_video_mode(&self, output_i: usize, mode: &mut OsVideoMode) {
        //TODO: return error?
        let mut outputs = self.outputs.borrow_mut();
        let (output, _efi_edid_opt) = &mut outputs[output_i];
        status_to_result((output.0.SetMode)(output.0, mode.id)).unwrap();

        // Update with actual mode information
        mode.width = output.0.Mode.Info.HorizontalResolution;
        mode.height = output.0.Mode.Info.VerticalResolution;
        mode.base = output.0.Mode.FrameBufferBase as u64;
    }

    fn best_resolution(&self, output_i: usize) -> Option<(u32, u32)> {
        let mut outputs = self.outputs.borrow_mut();
        let (output, efi_edid_opt) = outputs.get_mut(output_i)?;

        if let Some(efi_edid) = efi_edid_opt {
            let edid =
                unsafe { slice::from_raw_parts(efi_edid.0.Edid, efi_edid.0.SizeOfEdid as usize) };

            if edid.len() > 0x3D {
                return Some((
                    (edid[0x38] as u32) | (((edid[0x3A] as u32) & 0xF0) << 4),
                    (edid[0x3B] as u32) | (((edid[0x3D] as u32) & 0xF0) << 4),
                ));
            } else {
                log::warn!("EFI EDID too small: {}", edid.len());
            }
        }

        // Fallback to the current output resolution
        Some((
            output.0.Mode.Info.HorizontalResolution,
            output.0.Mode.Info.VerticalResolution,
        ))
    }

    fn get_key(&self) -> OsKey {
        //TODO: do not unwrap

        let mut index = 0;
        status_to_result((self.st.BootServices.WaitForEvent)(
            1,
            &self.st.ConsoleIn.WaitForKey,
            &mut index,
        ))
        .unwrap();

        let mut key = TextInputKey {
            ScanCode: 0,
            UnicodeChar: 0,
        };
        status_to_result((self.st.ConsoleIn.ReadKeyStroke)(
            self.st.ConsoleIn,
            &mut key,
        ))
        .unwrap();

        match key.ScanCode {
            0 => match key.UnicodeChar {
                8 => OsKey::Backspace,
                13 => OsKey::Enter,
                w => match char::from_u32(w as u32) {
                    Some(c) => OsKey::Char(c),
                    None => OsKey::Other,
                },
            },
            1 => OsKey::Up,
            2 => OsKey::Down,
            3 => OsKey::Right,
            4 => OsKey::Left,
            8 => OsKey::Delete,
            _ => OsKey::Other,
        }
    }

    fn clear_text(&self) {
        //TODO: why does this sometimes return InvalidParameter, but otherwise appear to work?
        let _ = status_to_result((self.st.ConsoleOut.ClearScreen)(self.st.ConsoleOut));
    }

    fn get_text_position(&self) -> (usize, usize) {
        (
            self.st.ConsoleOut.Mode.CursorColumn as usize,
            self.st.ConsoleOut.Mode.CursorRow as usize,
        )
    }

    fn set_text_position(&self, x: usize, y: usize) {
        // Ignore error because Tow-Boot appears to not implement this
        let _ = status_to_result((self.st.ConsoleOut.SetCursorPosition)(
            self.st.ConsoleOut,
            x,
            y,
        ));
    }

    fn set_text_highlight(&self, highlight: bool) {
        let attr = if highlight { 0x70 } else { 0x07 };
        status_to_result((self.st.ConsoleOut.SetAttribute)(self.st.ConsoleOut, attr)).unwrap();
    }
}

fn status_to_result(status: Status) -> Result<usize> {
    match status {
        Status(ok) if status.is_success() => Ok(ok),
        err => Err(err),
    }
}

/// 在 UEFI 环境中设置最大可用的文本模式
fn set_max_mode(output: &uefi::text::TextOutput) -> Result<()> {
    // max_i：存储最大模式的索引，max_w：最大宽度，max_h：最大高度
    let mut max_i = None;
    let mut max_w = 0;
    let mut max_h = 0;

    // 遍历所用可用的文本模式
    for i in 0..output.Mode.MaxMode as usize {
        let mut w = 0;
        let mut h = 0;
        if (output.QueryMode)(output, i, &mut w, &mut h).is_success() {
            if w >= max_w && h >= max_h {
                max_i = Some(i);
                max_w = w;
                max_h = h;
            }
        }
    }

    if let Some(i) = max_i {
        status_to_result((output.SetMode)(output, i))?;
    }

    Ok(())
}

/// 引导程序入口函数
// no_mangle 防止函数名重整
// 目标系统配置清单中的 entry:efi_main 指向 uefi_std crate 中 src/rt/start.rs 处的 efi_main 函数
#[no_mangle]
pub extern "C" fn main() -> Status {
    /// 获取系统表，包含UEFI固件提供的各种系统服务
    // Hdr：UEFI系统表头部信息
    // FirmwareVendor： 固件厂商的 UTF-16 字符串指针
    // FirmwareRevision：固件版本号
    // ConsoleInHandle：标准输入设备的句柄
    // ConsoleIn：指向标准输入接口
    // ConsoleOutHandle：标准输出设备的句柄
    // ConsoleOut：指向标准输出接口
    // ConsoleErrorHandle：错误输出设备的句柄
    // ConsoleError：指向错误输出接口
    // RuntimeServices：运行时服务表
    // BootServices：启动服务表
    // Entries：配置表数量
    // Configuration：指向配置表的指针
    let uefi = std::system_table();

    // 启动服务初始化看门狗
    let _ = (uefi.BootServices.SetWatchdogTimer)(0, 0, 0, ptr::null());

    if let Err(err) = set_max_mode(uefi.ConsoleOut) {
        println!("Failed to set max mode: {:?}", err);
    }

    if let Err(err) = arch::main() {
        panic!("App error: {:?}", err);
    }

    (uefi.RuntimeServices.ResetSystem)(ResetType::Cold, Status(0), 0, ptr::null());
}
