#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod interrupts;
pub mod panic;
pub mod serial;
pub mod testing;
pub mod vga_buffer;

pub fn init() {
    interrupts::init_idt();
}
