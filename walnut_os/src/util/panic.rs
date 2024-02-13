#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::println!("PANIC: {}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn test_panic(info: &core::panic::PanicInfo) -> ! {
    crate::println!("PANIC: {:#x?}", info);
    loop {}
}
