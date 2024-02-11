use crate::serial_println;

use super::ExceptionStackFrame;

pub extern "C" fn divide_by_zero_handler(sf: &ExceptionStackFrame) {
    serial_println!(
        "EXCEPTION: DIVIDE BY ZERO at {:#x}\n{:#x?}",
        sf.instruction_pointer,
        sf
    );
}

pub extern "C" fn breakpoint_handler(sf: &ExceptionStackFrame) {
    serial_println!(
        "EXCEPTION: BREAKPOINT at {:#x}\n{:#x?}",
        sf.instruction_pointer,
        sf
    );
}

pub extern "C" fn invalid_opcode_handler(sf: &ExceptionStackFrame) {
    serial_println!(
        "EXCEPTION: INVALID OPCODE at {:#x}\n{:#x?}",
        sf.instruction_pointer,
        sf
    );
}

pub extern "C" fn double_fault_handler(sf: &ExceptionStackFrame) {
    serial_println!(
        "EXCEPTION: DOUBLE FAULT at {:#x}\n{:#x?}",
        sf.instruction_pointer,
        sf
    );
    loop {}
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Debug)]
pub enum PageFaultErrorCode {
    ProtectionViolation = 1 << 0,
    CausedByWrite = 1 << 1,
    UserMode = 1 << 2,
    MalformedTable = 1 << 3,
    InstructionFetch = 1 << 4,
}

pub extern "C" fn page_fault_handler(sf: &ExceptionStackFrame, error_code: PageFaultErrorCode) {
    serial_println!(
        "EXCEPTION: PAGE FAULT with error: {:?}\n{:#x?}",
        error_code,
        sf
    );
}

macro_rules! exception_handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!("mov rdi, rsp",
                    "sub rsp, 8",
                    "call {}",
                    "add rsp, 8",
                    "iretq",
                  sym $name, options(noreturn));
                }
            }
        wrapper
    }
}}

macro_rules! exception_handler_w_error {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!("pop rsi",
                    "mov rdi, rsp",
                    "sub rsp, 8",
                    "call {}",
                    "add rsp, 8",
                    "iretq",
                  sym $name, options(noreturn));
                }
            }
        wrapper
    }
}}

#[test_case]
fn test_breakpoint_exception() {
    use core::arch::asm;
    unsafe {
        asm!("int3", options(nomem, nostack));
    }
}

#[test_case]
fn test_divide_by_zero_exception() {
    use core::arch::asm;
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

#[test_case]
fn test_invalid_opcode_exception() {
    use core::arch::asm;
    unsafe {
        asm!("ud2");
    }
}

#[test_case]
fn test_page_fault_exception() {
    unsafe {
        *(0xdeadbea8 as *mut u64) = 42;
    }
}
