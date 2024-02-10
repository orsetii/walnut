#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(walnut_os::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
pub use walnut_os::testing::test_runner;
use x86_64::instructions::interrupts::int3;

use core::panic::PanicInfo;
use walnut_os::{interrupts::init_idt, println, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Hello World{}", "!");
    println!("Walnut Initializing");
    walnut_os::init();

    int3();

    #[cfg(test)]
    test_main();

    println!("Entering OS loop");
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}
