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

use crate::{cpu::csr::ControlStatusRegister, info, mem::allocator::ALLOCATOR, HEAP_SIZE, HEAP_START, KERNEL_STACK_END, KERNEL_STACK_SIZE, KERNEL_STACK_START, TEXT_END, TEXT_START};

use super::{addr::VirtAddr, pages::{self, PAGE_ALLOCATOR}};

static mut KERNEL_PAGE_TABLE: *mut PageTable = core::ptr::null_mut();

fn k_pgtable() -> &'static mut PageTable {
    unsafe {
        &mut *KERNEL_PAGE_TABLE
    }
}


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

impl PageTableEntry {
    pub fn set_bits(&mut self, bits: usize) {
        self.0 = bits;
    }
}


pub fn init_hart() {
    todo!();
}


/// Currently this doesnt work 
/// we did ID map the the text section and stack section
/// All seemed fine, but i we attempt to 
/// print, we error on some operations on `a0`
/// so we disabled it
pub fn initialize() {

    unsafe {
        KERNEL_PAGE_TABLE = PAGE_ALLOCATOR.zalloc(1).expect("Unable to allocate page for kernel page table.") as *mut PageTable;
    }


    unsafe {
        id_map_range(
                    HEAP_START,
                    HEAP_START + ALLOCATOR.alloc_cnt() * 4096,
                    3 << 1);

        id_map_range(
                    TEXT_START,
                    TEXT_END,
                    7 << 1);

        id_map_range(
                    KERNEL_STACK_START,
                    KERNEL_STACK_END,
                    3 << 1);

        map(VirtAddr::from_bits(0x1000_0000), 0x1000_0000, 3 << 1, 0);

    }

        info!("ID Mapped heap from {:#0x} to {:#0x}", 
				HEAP_START,
                    HEAP_START + ALLOCATOR.alloc_cnt() * 4096,
        );
        info!("ID Mapped kernel stack from {:#0x} to {:#0x}", 
                    KERNEL_STACK_START,
                    KERNEL_STACK_END
        );
        info!("ID Mapped .text from {:#0x} to {:#0x}. PC at: {}", 
                    TEXT_START,
                    TEXT_END, ControlStatusRegister::Sepc.read()
        );
    unsafe {
        ControlStatusRegister::Satp.write((8 << 60) | KERNEL_PAGE_TABLE as usize >> 12);
    }

}

pub fn id_map_range(
	start: usize,
	end: usize,
	bits: isize)

{
	let mut memaddr = start & !(pages::PAGE_SIZE - 1);
	let num_kb_pages = (pages::align(end, 12)
		- memaddr)
		/ pages::PAGE_SIZE;

    info!("Mapping {} pages", num_kb_pages);

	// I named this num_kb_pages for future expansion when
	// I decide to allow for GiB (2^30) and 2MiB (2^21) page
	// sizes. However, the overlapping memory regions are causing
	// nightmares.
	for _ in 0..num_kb_pages {
        unsafe {
            map(VirtAddr::from_bits(memaddr as usize), memaddr as usize, bits, 0);
        }
        memaddr += 1 << 12;
	}
}


pub unsafe fn map(va: VirtAddr, pa: usize, flags: isize, lvl: usize) {

    assert!(flags & 0xe != 0);

    let tbl = k_pgtable();

    	let ppn = [
				// PPN[0] = paddr[20:12]
				(pa >> 12) & 0x1ff,
				// PPN[1] = paddr[29:21]
				(pa >> 21) & 0x1ff,
				// PPN[2] = paddr[55:30]
				(pa >> 30) & 0x3ff_ffff,
	];

    let mut v = &mut tbl.entries[va.lvl_idx(2)];

    for i in (lvl..2).rev() {


        if !v.get(PageTableEntry::VALID) {
            // TODO handle this error properly
            let page = PAGE_ALLOCATOR.zalloc(1).unwrap();
            v.set_bits((page as i64 >> 2) as usize);
            v.set(PageTableEntry::VALID, true);
        }
        let e = ((v.bits()as isize & !0x3ff) << 2) as *mut PageTableEntry;
        v = e.add(va.lvl_idx(i)).as_mut().unwrap();
    }

    let entry = (ppn[2] << 28) as isize |   // PPN[2] = [53:28]
			(ppn[1] << 19) as isize |   // PPN[1] = [27:19]
			(ppn[0] << 10) as isize |   // PPN[0] = [18:10]
			flags | 1;
    v.set_bits(entry as usize);
}


/// Get the PTE that corresponds 
/// to the virtual address.
pub fn walk(va: VirtAddr) -> Option<&'static PageTableEntry> {



    None
}
