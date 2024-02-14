#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(walnut_os::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[test_case]
fn test_println() {
    walnut_os::println!("walnut_os::println works!");
}
