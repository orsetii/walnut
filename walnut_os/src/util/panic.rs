#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn test_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
