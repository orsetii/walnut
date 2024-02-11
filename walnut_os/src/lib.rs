#![no_std]
#![cfg_attr(test, no_main)]
#![allow(internal_features)]
#![feature(
    custom_test_frameworks,
    abi_x86_interrupt,
    stmt_expr_attributes,
    core_intrinsics,
    naked_functions
)]
#![test_runner(testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod gdt;
pub mod interrupts;
pub mod panic;
pub mod serial;
pub mod testing;
pub mod vga_buffer;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}
