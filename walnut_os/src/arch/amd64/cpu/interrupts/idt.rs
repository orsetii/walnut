use super::entry::{Entry, EntryOptions, HandlerFunc, SegmentSelector};
use crate::println;
use x86_64::VirtAddr;

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
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
        use x86_64::instructions::tables::{lidt, DescriptorTablePointer};

        let ptr = DescriptorTablePointer {
            base: VirtAddr::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { lidt(&ptr) };
    }
}
