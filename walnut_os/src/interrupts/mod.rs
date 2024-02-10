use core::arch::asm;

use idt::Idt;
use lazy_static::lazy_static;

use crate::{println, serial_println};

pub mod idt;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.set_handler(0, divide_by_zero_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "C" fn divide_by_zero_handler() -> ! {
    serial_println!("EXCEPTION: DIVIDE BY ZERO");
    println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}

#[test_case]
fn test_breakpoint_exception() {
    unsafe {
        // TODO Impl handler
        // asm!("int3", options(nomem, nostack));
    }
}

#[test_case]
fn test_divide_by_zero_exception() {
    unsafe {
        // Move the dividend (4) into the RAX register
        // Zero out the RDX register (important for division)
        // Divide RAX by RDX (zero), triggering the fault
        asm!(
            "mov rax, 4   
            xor rdx, rdx 
            div rdx",
            options(nomem, nostack)
        );
    }
}
