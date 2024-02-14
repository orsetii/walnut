use crate::println;

mod gdt;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

/*
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt2() {
    IDT.load();
    unsafe {
        core::arch::asm!("int3");
    }
}
*/

lazy_static::lazy_static! {
    pub static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.divide_by_zero.set_handler_fn(div_by_zero_handler);
        idt
    };
}

//#[cfg(target_os = "windows")]
mod idt;
//#[cfg(target_os = "windows")]
#[allow(unconditional_panic)]
pub fn init_idt() {
    println!("Loading IDT...");
    IDT.load();

    unsafe {
        42 / 0;
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: idt::InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn div_by_zero_handler(stack_frame: idt::InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    loop {}
}
