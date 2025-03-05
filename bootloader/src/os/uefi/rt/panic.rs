use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    print!("SETUP PANIC: {}", info);

    loop {}
}
