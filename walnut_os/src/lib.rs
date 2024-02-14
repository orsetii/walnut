#![no_std]
#![no_main]
#![feature(
    custom_test_frameworks,
    const_mut_refs,
    const_ptr_as_ref,
    const_option,
    abi_x86_interrupt,
    naked_functions
)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod arch;
pub mod tests;
pub mod util;
