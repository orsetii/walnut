use crate::println;

#[derive(Debug)]
#[repr(C)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

pub extern "C" fn divide_by_zero_handler(sf: &ExceptionStackFrame) {
    println!(
        "EXCEPTION: DIVIDE BY ZERO at {:#x}\n{:#x?}",
        sf.instruction_pointer, sf
    );
    crate::vga_println!(
        "EXCEPTION: DIVIDE BY ZERO at {:#x}\n{:#x?}",
        sf.instruction_pointer,
        sf
    );
}

pub extern "C" fn breakpoint_handler(sf: &ExceptionStackFrame) {
    println!(
        "EXCEPTION: BREAKPOINT at {:#x}\n{:#x?}",
        sf.instruction_pointer, sf
    );
}

pub extern "C" fn invalid_opcode_handler(sf: &ExceptionStackFrame) {
    println!(
        "EXCEPTION: INVALID OPCODE at {:#x}\n{:#x?}",
        sf.instruction_pointer, sf
    );
}

pub extern "C" fn double_fault_handler(sf: &ExceptionStackFrame) {
    println!(
        "EXCEPTION: DOUBLE FAULT at {:#x}\n{:#x?}",
        sf.instruction_pointer, sf
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
    println!(
        "EXCEPTION: PAGE FAULT with error: {:?}\n{:#x?}",
        error_code, sf
    );
}

macro_rules! exception_handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!(

                    save_scratch_registers!(),
                    "mov rdi, rsp",
                    "sub rsp, 8",
                    "call {}",
                    restore_scratch_registers!(),
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
                asm!(
                    save_scratch_registers!(),
                    "mov rsi, [rsp + 9*8]",
                    "mov rdi, rsp",
                    "add rdi, 10*8",
                    "sub rsp, 8",
                    "call {}",
                    "add rsp, 8",
                    restore_scratch_registers!(),
                    "iretq",
                  sym $name, options(noreturn));
                }
            }
        wrapper
    }
}}

macro_rules! save_scratch_registers {
    () => {
        "push rax
              push rcx
              push rdx
              push rsi
              push rdi
              push r8
              push r9
              push r10
              push r11
        "
    };
}

macro_rules! restore_scratch_registers {
    () => {
        "pop r11
              pop r10
              pop r9
              pop r8
              pop rdi
              pop rsi
              pop rdx
              pop rcx
              pop rax
            "
    };
}

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
