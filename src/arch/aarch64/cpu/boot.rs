global_asm!(include_str!("boot.s"));

/// The entry point of the kernel.
#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    crate::kernel_init()
}
