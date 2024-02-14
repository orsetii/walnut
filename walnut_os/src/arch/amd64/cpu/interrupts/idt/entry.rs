#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Entry {
    isr_ptr_low: u16,
    kernel_cs: SegmentSelector,
    attributes: EntryAttributes,
    isr_ptr_mid: u16,
    isr_ptr_high: u32,
    _reserved: u32,
}

impl Entry {
    pub fn new(segment: SegmentSelector, func: super::HandlerFunc) -> Self {
        let addr = func as usize;
        let e = Self {
            isr_ptr_low: addr as u16,
            isr_ptr_mid: (addr >> 16) as u16,
            isr_ptr_high: (addr >> 32) as u32,
            kernel_cs: segment,
            attributes: EntryAttributes::new(),
            _reserved: 0,
        };
        crate::println!("{:#x?}", e);
        e
    }
    pub const fn missing() -> Self {
        Self {
            isr_ptr_low: 0,
            kernel_cs: SegmentSelector(0),
            attributes: EntryAttributes::base(),
            isr_ptr_mid: 0,
            isr_ptr_high: 0,
            _reserved: 0,
        }
    }
    pub fn is_empty(e: Self) -> bool {
        e.isr_ptr_high == 0 && e.isr_ptr_mid == 0 && e.isr_ptr_low == 0
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
struct EntryAttributes(u16);

impl EntryAttributes {
    pub fn new() -> Self {
        let mut ea = Self::base();
        ea.set_bits(0b111, 9..12);
        ea.set_present(true).disable_interrupts(false);
        ea
    }

    /// A zero-ed `EntryAttributes`, with only the
    /// *must-set* bits set.
    const fn base() -> Self {
        Self(0x0)
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.set_bit(present, 15);
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.set_bit(!disable, 8);
        self
    }

    pub fn set_priv_level(&mut self, ring_lvl: u16) -> &mut Self {
        self.set_bits(ring_lvl, 13..15);
        self
    }

    pub fn set_stack_index(&mut self, idx: u16) -> &mut Self {
        self.set_bits(idx, 0..3);
        self
    }

    const fn set_bit(&mut self, val: bool, pos: u16) {
        assert!(pos < 16, "Bit position out of range for u16");
        let mask = 1 << pos;

        if val {
            self.0 |= mask; // Set the bit using OR if 'should_set' is true
        } else {
            self.0 &= !mask; // Clear the bit using AND with
                             // inverted mask if 'should_set' is false
        }
    }

    const fn get_bit(&self, pos: u16) -> bool {
        assert!(pos < 16, "Bit position out of range for u16");
        let mask = 1 << pos;

        (self.0 & mask) != 0
    }

    fn set_bits(&mut self, bits: u16, range: core::ops::Range<u16>) {
        assert!(
            range.start < 16 && range.end <= 16,
            "Invalid bit range for u16"
        );
        assert!(
            range.end - range.start <= 16,
            "Range too large for bits parameter"
        );

        for (i, bit_pos) in range.enumerate() {
            let bit_value = (bits >> i) & 1 == 1; // Extract the relevant bit from 'bits'
            self.set_bit(bit_value, bit_pos);
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct SegmentSelector(pub u16);
