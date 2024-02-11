use core::arch::asm;

use lazy_static::lazy_static;

use exceptions::*;
use x86_64::structures::idt::InterruptStackFrame;

use crate::gdt;

#[macro_use]
mod exceptions;
pub mod idt;

lazy_static! {
    /*
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.set_handler(0, exception_handler!(divide_by_zero_handler));
        idt.set_handler(3, exception_handler!(breakpoint_handler));
        idt.set_handler(6, exception_handler!(invalid_opcode_handler));
        idt.set_handler(8, exception_handler!(double_fault_handler));
        idt.set_handler(14, exception_handler_w_error!(page_fault_handler));
        idt
    };
    */
        static ref IDT: x86_64::structures::idt::InterruptDescriptorTable = {
        let mut idt = x86_64::structures::idt::InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
        }

        idt
    };
}

#[derive(Debug)]
#[repr(C)]
struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

pub fn init_idt() {
    IDT.load();
}

pub extern "x86-interrupt" fn double_fault_handler(sf: InterruptStackFrame, ecode: u64) -> ! {
    crate::serial_println!(
        "EXCEPTION: DOUBLE FAULT at {:#x}\n{:#x?}",
        sf.instruction_pointer,
        sf
    );
    loop {}
}
