#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::vga_println!("PANIC: {:#?}", info);
    crate::println!("PANIC: {:#?}", info);
    loop {}
}

#[cfg(test)]
pub fn test_panic_handler(info: &core::panic::PanicInfo) -> ! {
    crate::vga_println!("PANIC: {:#?}", info);
    crate::println!("PANIC: {:#?}", info);
    loop {}
}
