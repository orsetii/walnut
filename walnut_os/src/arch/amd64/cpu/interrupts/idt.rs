
use super::entry::{Entry, EntryOptions, HandlerFunc};

#[repr(C, packed(2))]
pub struct Idtr {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: u64,
}

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
#[repr(C)]
pub struct Idt([Entry; 16]);

impl Idt {
    pub fn new() -> Self {
        Idt([Entry::missing(); 16])
    }

    pub fn set_handler(&mut self, index: usize, handler: HandlerFunc) -> &mut EntryOptions {
        self.0[index].set_handler_fn(handler)
    }

    pub fn load(&'static self) {
        use core::mem::size_of;

        let ptr = Idtr {
            base: self as *const _ as u64,
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { w_lidt(&ptr) };
    }
}

unsafe fn w_lidt(idt: &Idtr) {
    unsafe {
        core::arch::asm!("lidt [{}]", in(reg) idt, options(readonly, nostack, preserves_flags));
    }
}
