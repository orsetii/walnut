#![no_std]
#![no_main]
#![feature(custom_test_frameworks, const_mut_refs, const_ptr_as_ref, const_option)]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    walnut_os::println!("Walnut OS booting...");
    walnut_os::arch::initialize();

    loop {}
}
