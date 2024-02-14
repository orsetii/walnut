#![no_std]
#![no_main]
#![feature(custom_test_frameworks, const_mut_refs, const_ptr_as_ref, const_option)]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    walnut_os::println!("Walnut OS booting...");
    walnut_os::arch::initialize();

    bp();
    page_fault();
    divide_by_zero();

    walnut_os::println!("Walnut OS booting...");
    loop {}
}

fn divide_by_zero() {
    unsafe { core::arch::asm!("mov dx, 0", "div dx") }
}

fn bp() {
    unsafe { core::arch::asm!("int3") }
}

fn page_fault() {
    // provoke a page fault
    unsafe {
        *(0xdeadbea8 as *mut u64) = 42;
    }
}
