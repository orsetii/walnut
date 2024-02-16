#![no_std]
#![feature(
    panic_info_message,
    allocator_api,
    alloc_error_handler,
    custom_test_frameworks
)]

#[macro_use]
extern crate alloc;

// This is experimental and requires alloc_prelude as a feature
use alloc::boxed::Box;
use io::serial::BS;

use crate::mem::{
    kalloc,
    page::{align_val, map, print_page_allocations, virt_to_phys, EntryBits, Table, PAGE_SIZE},
};

#[macro_use]
pub mod io;

pub mod cpu;
pub mod mem;
pub mod sync;

#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Aborting: ");
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        println!("no information available.");
    }
    abort();
}
#[no_mangle]
extern "C" fn abort() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

// ///////////////////////////////////
// / CONSTANTS
// ///////////////////////////////////
// const STR_Y: &str = "\x1b[38;2;79;221;13m✓\x1b[m";
// const STR_N: &str = "\x1b[38;2;221;41;13m✘\x1b[m";

// The following symbols come from asm/mem.S. We can use
// the symbols directly, but the address of the symbols
// themselves are their values, which can cause issues.
// Instead, I created doubleword values in mem.S in the .rodata and .data
// sections.
extern "C" {
    static TEXT_START: usize;
    static TEXT_END: usize;
    static DATA_START: usize;
    static DATA_END: usize;
    static RODATA_START: usize;
    static RODATA_END: usize;
    static BSS_START: usize;
    static BSS_END: usize;
    static KERNEL_STACK_START: usize;
    static KERNEL_STACK_END: usize;
    static HEAP_START: usize;
    static HEAP_SIZE: usize;
    static mut KERNEL_TABLE: usize;
}

/// Identity map range
/// Takes a contiguous allocation of memory and maps it using PAGE_SIZE
/// This assumes that start <= end
pub fn id_map_range(root: &mut Table, start: usize, end: usize, bits: i64) {
    let mut memaddr = start & !(PAGE_SIZE - 1);
    let num_kb_pages = (align_val(end, 12) - memaddr) / PAGE_SIZE;

    // I named this num_kb_pages for future expansion when
    // I decide to allow for GiB (2^30) and 2MiB (2^21) page
    // sizes. However, the overlapping memory regions are causing
    // nightmares.
    for _ in 0..num_kb_pages {
        map(root, memaddr, memaddr, bits, 0);
        memaddr += 1 << 12;
    }
}

#[no_mangle]
extern "C" fn kinit() -> usize {
    // We created kinit, which runs in super-duper mode
    // 3 called "machine mode".
    mem::init()
}

#[no_mangle]
extern "C" fn kmain() {
    // kmain() starts in supervisor mode. So, we should have the trap
    // vector setup and the MMU turned on when we get here.
    // Create a new scope so that we can test the global allocator and
    // deallocator
    {
        // We have the global allocator, so let's see if that works!
        let k = Box::<u32>::new(100);
        println!("Boxed value = {}", *k);
        kalloc::print_table();
        // The following comes from the Rust documentation:
        // some bytes, in a vector
        let sparkle_heart = vec![240, 159, 146, 150];
        // We know these bytes are valid, so we'll use `unwrap()`.
        let sparkle_heart = alloc::string::String::from_utf8(sparkle_heart).unwrap();
        println!("String = {}", sparkle_heart);
    }
    // If we get here, the Box, vec, and String should all be freed since
    // they go out of scope. This calls their "Drop" trait.
    // Now see if we can read stuff:
    // Usually we can use #[test] modules in Rust, but it would convolute
    // the task at hand, and it requires us to create the testing harness
    // since the embedded testing system is part of the "std" library.
    while let Some(c) = io::serial::SERIAL.read_char_non_blocking() {
        match c {
            BS => {
                // This is a backspace, so we
                // essentially have to write a space and
                // backup again:
                print!("{} {}", 8 as char, 8 as char);
            }
            '\n' | '\r' => {
                // Newline or carriage-return
                println!();
            }
            _ => {
                print!("{}", c as char);
            }
        }
    }
}
