#![no_std]
#![no_main]
#![feature(custom_test_frameworks, const_mut_refs, const_ptr_as_ref, const_option)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod arch;
pub mod util;

#[cfg(test)]
pub mod tests;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to {}!", "Walnut");
    #[cfg(test)]
    test_main();

    loop {}
}
