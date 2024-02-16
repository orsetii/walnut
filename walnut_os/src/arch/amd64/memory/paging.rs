#[repr(C, align(4096))]
pub struct PageTable(pub [PageTableEntry; 512]);

#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub fn present(&self) -> bool {
        (self.0 & 1 << 0) == 1
    }

    pub fn writable(&self) -> bool {
        (self.0 & 1 << 1) == (1 << 1)
    }

    pub fn kernel_only(&self) -> bool {
        (self.0 & 1 << 2) == (1 << 2)
    }

    pub fn write_through(&self) -> bool {
        (self.0 & 1 << 3) == (1 << 3)
    }

    pub fn cache_disabled(&self) -> bool {
        (self.0 & 1 << 4) == (1 << 4)
    }

    pub fn is_used(&self) -> bool {
        (self.0 & 1 << 5) == (1 << 5)
    }

    pub fn is_dirty(&self) -> bool {
        (self.0 & 1 << 6) == (1 << 6)
    }

    /// must be 0 in P1 and P4
    /// creates a 1 GiB page in P3
    /// creates a 2 MiB page in P2
    pub fn special_page(&self) -> bool {
        (self.0 & 1 << 7) == (1 << 7)
    }

    /// Page is not flushed from caches
    /// on address space switch
    /// # Usage
    /// PGE bit of CR4 register must be set
    pub fn global(&self) -> bool {
        (self.0 & 1 << 8) == (1 << 8)
    }

    /// page aligned 52-bit physical address
    /// of the frame or the next page table
    pub fn phys_addr(&self) -> u64 {
        (self.0 >> 12) & 0x7FFFFFFF
    }

    pub fn nx(&self) -> bool {
        (self.0 & 1 << 63) == (1 << 63)
    }
}
