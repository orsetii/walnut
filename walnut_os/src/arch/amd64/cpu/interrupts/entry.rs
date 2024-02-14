use bit_field::BitField;

use crate::println;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    #[inline]
    pub const fn new(index: u16, rpl: PrivilegeLevel) -> SegmentSelector {
        SegmentSelector(index << 3 | rpl as u16)
    }

    #[allow(dead_code)]
    /// Returns the GDT index.
    #[inline]
    pub fn index(self) -> u16 {
        self.0 >> 3
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PrivilegeLevel {
    /// Privilege-level 0 (most privilege): This level is used by critical system-software
    /// components that require direct access to, and control over, all processor and system
    /// resources. This can include BIOS, memory-management functions, and interrupt handlers.
    Ring0 = 0,

    /// Privilege-level 1 (moderate privilege): This level is used by less-critical system-
    /// software services that can access and control a limited scope of processor and system
    /// resources. Software running at these privilege levels might include some device drivers
    /// and library routines. The actual privileges of this level are defined by the
    /// operating system.
    Ring1 = 1,

    /// Privilege-level 2 (moderate privilege): Like level 1, this level is used by
    /// less-critical system-software services that can access and control a limited scope of
    /// processor and system resources. The actual privileges of this level are defined by the
    /// operating system.
    Ring2 = 2,

    /// Privilege-level 3 (least privilege): This level is used by application software.
    /// Software running at privilege-level 3 is normally prevented from directly accessing
    /// most processor and system resources. Instead, applications request access to the
    /// protected processor and system resources by calling more-privileged service routines
    /// to perform the accesses.
    Ring3 = 3,
}

pub type HandlerFunc = extern "C" fn() -> !;

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
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
    pub fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
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

    pub fn missing() -> Self {
        Entry {
            segment_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            isd_ptr_low: 0,
            isd_ptr_mid: 0,
            isd_ptr_high: 0,
            options: EntryOptions::new(),
            _reserved: 0,
        }
    }
    pub fn set_handler_fn(&mut self, handler: HandlerFunc) -> &mut EntryOptions {
        let pointer = handler as u64;
        self.isd_ptr_low = pointer as u16;
        self.isd_ptr_mid = (pointer >> 16) as u16;
        self.isd_ptr_high = (pointer >> 32) as u32;
        self.segment_selector = get_cs();
        self.options.set_present(true);
        &mut self.options
    }
}

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EntryOptions(u16);

#[allow(dead_code)]
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

#[inline]
fn get_cs() -> SegmentSelector {
    let segment: u16;
    unsafe {
        core::arch::asm!("mov {0:x}, cs", out(reg) segment, options(nomem, nostack, preserves_flags));
    }
    println!("Got CS segment: {}", segment);
    SegmentSelector(segment)
}
