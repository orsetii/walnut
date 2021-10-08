use cortex_a::asm;

/// Pause execution on the core
#[inline]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe();
    }
}
