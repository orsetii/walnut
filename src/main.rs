#![no_std]
#![no_main]
#![feature(custom_test_frameworks, asm, const_mut_refs)]
#![test_runner(aos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use aos::println;
use aos::pith::memory;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::VirtAddr;
use x86_64::structures::paging::{Page, Translate};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

entry_point!(kmain);

fn kmain(boot_info: &'static BootInfo) -> ! {
    use aos::pith::memory::BootInfoFrameAllocator;

    println!("hello bitches");
    aos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    aos::pith::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap intialization failed...");
    let x = Box::new(64);
    println!("alloced '64' at {:p}", x);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    aos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    aos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    aos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
