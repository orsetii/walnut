use core::panic::PanicInfo;

use crate::println;

#[panic_handler]
fn panic_handler<'a>(info: &'a PanicInfo) -> ! {
    println!("PANIC!!!\n {:#x?}", info);

    loop {}
}
