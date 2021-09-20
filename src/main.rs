#![no_std]
#![no_main]
#![feature(asm, abi_efiapi, llvm_asm, bool_to_option)]
#![feature(panic_info_message, alloc_error_handler)]
#![feature(abi_x86_interrupt)]
#![allow(clippy::missing_safety_doc)]
pub mod acpi;
pub mod arch;
#[doc(hidden)]
mod core_fns;
pub mod cpu;
#[doc(hidden)]
mod efi;
mod error;
pub mod mm;
pub mod paging;


pub use crate::arch::idt;
pub mod serial;

pub fn kmain(mut memory_map: mm::RangeSet) {
    paging::init(memory_map);
    panic!("reached end of kmain()");
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    println!("!!! PANIC !!!");
    if let Some(loc) = _info.location() {
        println!("Location: {}", loc);
    }
    if let Some(msg) = _info.message() {
        println!("Message:  {}", msg);
    }
    dump_state!();
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
