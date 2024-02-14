use core::{arch::asm, mem::size_of};

use entry::Entry;

use entry::SegmentSelector;

use crate::println;
mod entry;




#[derive(Debug)]
#[repr(C)]
#[repr(align(16))]
pub struct Idt {
   pub divide_by_zero: Entry,
    entries: [Entry; 256-1],
}


impl Idt {
    pub fn new() -> Self {
   Idt {
       divide_by_zero: Entry::missing(),
        entries: [Entry::missing(); 256 - 1],
    }
    }
    pub fn load(&self) {
        unsafe {
            self.lidt();
        }
    }

    
    pub unsafe fn lidt(&self) {
        let idtr = Idtr {
            base: self as *const _ as u64,
            limit: (size_of::<Self>() - 1) as u16,
        };
        unsafe {
            asm!("lidt [{}]", 
                in(reg) &idtr, 
                options(readonly, nostack, preserves_flags));
        }
    }
}



#[repr(C, packed)]
pub struct Idtr {
    base: u64,
    limit: u16,
}

/// Represents the interrupt stack frame pushed by the CPU on interrupt or exception entry.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct InterruptStackFrame {
    inner: InterruptStackFrameInner,
}

/// Represents the interrupt stack frame pushed by the CPU on interrupt or exception entry.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct InterruptStackFrameInner {
    /// This value points to the instruction that should
    /// be executed when the interrupt handler returns.
    /// For most interrupts, this value points to the instruction immediately
    /// following the last executed instruction.
    /// However, for some exceptions (e.g., page faults), this value points to
    /// the faulting instruction, so that the instruction is restarted on
    /// return. See the documentation of the [`InterruptDescriptorTable`] fields for details
    pub instruction_pointer: usize,
    /// The code segment selector, padded with zeros.
    pub code_segment: u64,
    /// The flags register before the interrupt handler was invoked.
    pub cpu_flags: u64,
    /// The stack pointer at the time of the interrupt.
    pub stack_pointer: usize,
    /// The stack segment descriptor at the time of the interrupt (often zero in 64-bit mode).
    pub stack_segment: u64,
}

impl core::fmt::Debug for InterruptStackFrameInner {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        struct Hex(u64);
        impl core::fmt::Debug for Hex {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        let mut s = f.debug_struct("InterruptStackFrame");
        s.field("instruction_pointer", &self.instruction_pointer);
        s.field("code_segment", &self.code_segment);
        s.field("cpu_flags", &Hex(self.cpu_flags));
        s.field("stack_pointer", &self.stack_pointer);
        s.field("stack_segment", &self.stack_segment);
        s.finish()
    }
}

impl core::ops::Deref for InterruptStackFrame {
    type Target = InterruptStackFrameInner;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::fmt::Debug for InterruptStackFrame {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

type HandlerFunc = extern "x86-interrupt" fn(_: InterruptStackFrame);
