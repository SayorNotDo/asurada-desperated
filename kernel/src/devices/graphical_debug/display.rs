use core::{ptr, slice};

use alloc::boxed::Box;

pub(super) struct Display {
    pub(super) width: usize,       // 显示区域有效宽度（像素）
    pub(super) height: usize,      // 显示区域有效高度（像素）
    pub(super) stride: usize,      // 每行实际像素数（含对齐填充）
    onscreen_ptr: *mut u32,        // 显存原始指针（直接映射到硬件）
    offscreen: Option<Box<[u32]>>, // 离屏缓冲区（双缓冲），避免直接修改显存导致的画面撕裂
}

// 显式标记为线程安全
unsafe impl Send for Display {}

impl Display {
    pub(super) fn new(
        width: usize,
        height: usize,
        stride: usize,
        onscreen_ptr: *mut u32,
    ) -> Display {
        unsafe {
            ptr::write_bytes(onscreen_ptr, 0, stride * height);
        }

        Display {
            width,
            height,
            stride,
            onscreen_ptr,
            offscreen: None,
        }
    }

    // 离屏缓冲区初始化
    pub(super) fn heap_init(&mut self) {
        let onscreen =
            unsafe { slice::from_raw_parts(self.onscreen_ptr, self.stride * self.height) }; // 清空显存
        self.offscreen = Some(onscreen.to_vec().into_boxed_slice()); // 复制显存到堆
    }

    pub(super) fn data_mut(&mut self) -> *mut u32 {
        match &mut self.offscreen {
            Some(offscreen) => offscreen.as_mut_ptr(), // 离屏缓冲区指针
            None => self.onscreen_ptr,                 // 直接显存指针（无双缓冲时）
        }
    }

    // 同步显存方法，非安全，依赖调用者保证 x + w <= stride 且 y + h <= height，否则导致内存越界
    pub(super) unsafe fn sync(&mut self, x: usize, y: usize, w: usize, mut h: usize) {
        if let Some(offscreen) = &self.offscreen {
            let mut offset = y * self.stride + x; // 计算起始偏移
            while h > 0 {
                // 逐行复制离屏数据到显存，适配stride可能大于width的对齐场景
                ptr::copy(
                    offscreen.as_ptr().add(offset),
                    self.onscreen_ptr.add(offset),
                    w,
                );
                offset += self.stride;
                h -= 1;
            }
        }
    }
}
