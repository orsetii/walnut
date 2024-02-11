use bit_field::BitField;
use x86_64::{instructions::segmentation, structures::gdt::SegmentSelector, VirtAddr};
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub struct Idt([Entry; 16]);

impl Idt {
    pub fn new() -> Self {
        Idt([Entry::missing(); 16])
    }

    pub fn set_handler(&mut self, entry_idx: u8, handler: HandlerFunc) {
        self.0[entry_idx as usize] = Entry::new(segmentation::cs(), handler);
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

/*
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SegmentSelector(u16);

impl SegmentSelector {
    #[inline]
    pub const fn new(index: u16, rpl: u16) -> SegmentSelector {
        SegmentSelector(index << 3 | rpl)
    }

    /// Returns the GDT index.
    #[inline]
    pub fn index(self) -> u16 {
        self.0 >> 3
    }
}

*/

pub type HandlerFunc = extern "C" fn() -> !;

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry {
    /// The first 16 bits of the address of the entry point of the
    /// Interrupt Service Routine
    isd_ptr_low: u16,
    pub segment_selector: SegmentSelector,
    pub options: EntryOptions,
    isd_ptr_mid: u16,
    isd_ptr_high: u32,
    _reserved: u32,
}

impl Entry {
    fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
        let pointer = handler as u64;
        Entry {
            segment_selector: gdt_selector,
            isd_ptr_low: pointer as u16,
            isd_ptr_mid: (pointer >> 16) as u16,
            isd_ptr_high: (pointer >> 32) as u32,
            options: EntryOptions::new(),
            _reserved: 0,
        }
    }

    fn missing() -> Self {
        Entry {
            segment_selector: SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0),
            isd_ptr_low: 0,
            isd_ptr_mid: 0,
            isd_ptr_high: 0,
            options: EntryOptions::new(),
            _reserved: 0,
        }
    }
}

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EntryOptions(u16);

impl EntryOptions {
    fn minimal() -> Self {
        let mut options = 0;
        options.set_bits(9..12, 0b111); // 'must-be-one' bits
        EntryOptions(options)
    }

    fn new() -> Self {
        let mut options = Self::minimal();
        options.set_present(true).disable_interrupts(true);
        options
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        self.0.set_bits(13..15, dpl);
        self
    }

    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0.set_bits(0..3, index);
        self
    }
}
