//! Mapping of physical memory to virtual memory.
//!
//! # TODOs
//!
//! sfence VMA functions
//! make satp in init fn 
//! map fn 
//! unmap fn 
//! perform id mappings
use mycelium_bitfield::bitfield;



#[repr(C)]
pub struct PageTable {
    entries: [PageTableEntry; 512]
}

bitfield! {
    pub struct PageTableEntry<usize> {
        pub const VALID: bool;
        pub const READ_PERMISSIONS: bool;
        pub const WRITE_PERMISSIONS: bool;
        pub const EXEC_PERMISSIONS: bool;
        pub const USER_MODE_ACCISSIBLE: bool;
        pub const GLOBAL_MAPPING: bool;
        pub const ACCESSED: bool;
        pub const DIRTY: bool;
        const _RESERVED = 2;
        pub const PPN = 44;
    }
}
