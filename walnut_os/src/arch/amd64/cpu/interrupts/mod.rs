use core::arch::asm;

use lazy_static::lazy_static;

#[macro_use]
mod exceptions;

mod entry;
pub mod gdt;
pub mod idt;

use exceptions::*;
lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(0, exception_handler!(divide_by_zero_handler));
        idt.set_handler(3, exception_handler!(breakpoint_handler));
        idt.set_handler(6, exception_handler!(invalid_opcode_handler));
        idt.set_handler(8, exception_handler!(double_fault_handler))
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        idt.set_handler(14, exception_handler_w_error!(page_fault_handler));
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
