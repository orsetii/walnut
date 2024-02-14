use crate::println;

mod gdt;
mod idt;

pub fn init_idt() {
    println!("Loading IDT...");
    idt::IDT.load();

    unsafe {
        core::arch::asm!("int3");
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: idt::InterruptStackFrame) {
    //println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn div_by_zero_handler(stack_frame: idt::InterruptStackFrame) {
    //println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    loop {}
}
