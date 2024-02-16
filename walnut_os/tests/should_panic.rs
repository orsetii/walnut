#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(walnut_os::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use walnut_os::print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[test_case]
//#[should_panic]
fn should_fail() {
    print!("should_panic::should_fail...\t");
    // TODO: our panic handler currently just loops
    // meaning the test never returns, so we can never
    // check that it DID panic or not.
    // once we handle panics, we can run these kind of tests!
    //assert_eq!(0, 1);
}
