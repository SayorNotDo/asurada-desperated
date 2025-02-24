//! Intrinsics for panic handling
use core::panic::PanicInfo;

/// Required to handle panics
#[cfg(not(test))]
#[panic_handler]
fn rust_begin_unwind(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", info);

    unsafe {
        stack_trace();
    }

    loop {}
}

#[inline(never)]
pub unsafe fn stack_trace() {}
