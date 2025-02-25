/*
 * 图形化调试系统的初始化模块
 * 帧缓冲区配置解析：从环境变量中读取硬件帧缓冲区参数（物理地址、虚拟地址、分辨率等）
 * 显示系统初始化：创建 Display 和 DebugDisplay 实例，用于图形输出
 * 双缓冲管理：通过 init_heap() 初始化离屏缓冲区
 */

use spin::Mutex;

pub use self::debug::DebugDisplay;
use self::display::Display;

pub mod debug;
pub mod display;

// 全局状态管理
pub static DEBUG_DISPLAY: Mutex<Option<DebugDisplay>> = Mutex::new(None);
pub static FRAMEBUFFER: Mutex<(usize, usize, usize)> = Mutex::new((0, 0, 0));

#[allow(unused)]
pub fn init(env: &[u8]) {
    println!("Starting graphical debug");

    let mut phys = 0;
    let mut virt = 0;
    let mut width = 0;
    let mut height = 0;
    let mut stride = 0;

    for line in str::from_utf8(env).unwrap_or("").lines() {
        let mut parts = line.splitn(2, '=');
        let name = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");
        match name {
            "FRAMEBUFFER_ADDR" => phys = usize::from_str_radix(value, 16).unwrap_or(0),
            "FRAMEBUFFER_VIRT" => virt = usize::from_str_radix(value, 16).unwrap_or(0),
            "FRAMEBUFFER_WIDTH" => width = usize::from_str_radix(value, 16).unwrap_or(0),
            "FRAMEBUFFER_HEIGHT" => height = usize::from_str_radix(value, 16).unwrap_or(0),
            "FRAMEBUFFER_STRIDE" => stride = usize::from_str_radix(value, 16).unwrap_or(0),
            _ => (),
        }
    }

    *FRAMEBUFFER.lock() = (phys, virt, height * stride * 4);

    if phys == 0 || virt == 0 || width == 0 || height == 0 || stride == 0 {
        println!("Framebuffer not found");
        return;
    }
    println!(
        "Framebuffer {}x{} stride {} at {:X} mapped to {:X}",
        width, height, stride, phys, virt
    );
    {
        let display = Display::new(width, height, stride, virt as *mut u32);
        let debug_display = DebugDisplay::new(display);
        *DEBUG_DISPLAY.lock() = Some(debug_display);
    }
}

#[allow(unused)]
pub fn init_heap() {
    if let Some(debug_display) = &mut *DEBUG_DISPLAY.lock() {
        debug_display.display.heap_init();
    }
}

#[allow(unused)]
pub fn finish() {
    DEBUG_DISPLAY.lock().take();
    println!("Finished graphical debug");
}
