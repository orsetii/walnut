use core::panic::PanicInfo;

use crate::{cpu::util::my_hart, println};

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("PANIC IN HART#{}!!!\n {:#x?}", my_hart(), info);

    loop {}
}
