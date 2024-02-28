use mycelium_bitfield::bitfield;

use crate::{info, println, HEAP_START};

pub const PAGE_SIZE: usize = 4096;

#[repr(C, align(4096))]
pub struct Page {
    pub data: [u8; PAGE_SIZE],
}

bitfield! {
    pub struct PageListNode<u8> {
        pub const TAKEN: bool;
        pub const LAST: bool;
    }
}

pub static mut PAGE_ALLOCATOR: PageAllocator = PageAllocator { alloc_start: 0 };

pub struct PageAllocator {
    pub alloc_start: usize,
}

impl PageAllocator {
    /// Intializes the page allocator
    ///
    /// # Safety
    /// TODO
    pub unsafe fn init(&mut self) {
        assert!(self.alloc_start == 0);

        // Perform the initialization here,
        // instead of messing around sticking the page allocator in a spinlock etc,
        // due to restrictions on global static muts
        const PAGE_ORDER: usize = 12;

        zero_bytes(HEAP_START as *const u8, page_count());

        self.alloc_start = align(HEAP_START + page_count(), PAGE_ORDER);
        crate::info!("Allocation start set to {:#0x}", self.alloc_start);
    }

    pub fn zalloc(&self, n: usize) -> Option<*const Page> {
        let pg_ptr = self.alloc(n)?;

        let small_ptr = pg_ptr as *mut u64;
        // cast the page pointer as a u64 so 
        // we can can perform the zeroing faster



        for i in 0..(n * PAGE_SIZE )/8 {
            unsafe {
                *small_ptr.add(i) = 0;
            }
        }


        Some(pg_ptr)
    }

    pub fn alloc(&self, n: usize) -> Option<*const Page> {
        assert!(self.alloc_start != 0);

        let node = unsafe { HEAP_START } as *mut PageListNode;
        for i in 0..page_count() - n {
            if unsafe { !node_is_taken(node.add(i)) } && self.has_contig_space(node, n, i) {
                for pg_idx in i..i+n {
                    unsafe {
                        (*node.add(pg_idx)).set(PageListNode::TAKEN, true);
                    }
                }
                unsafe { (*node.add((i+n)-1)).set(PageListNode::LAST, true); }
                return Some((self.alloc_start + (i * PAGE_SIZE)) as *const Page);
            }
        }
        crate::warn!("No contiguous space for {} page allocation was found!", n);
        None
    }

    pub fn dealloc<T>(&mut self, p: *const T) {

        // we assume that the given pointer is at the base
        // of the page allocation. if it is not, then there 
        // WILL be pages that are left dangling as 'taken'
        let mut node = self.node_for_address(p);
        while node_is_taken(node) && !node_is_last(node) {
            unsafe {
                (*node).set(PageListNode::TAKEN, false);
                node = node.add(1);
            }
        }
        assert!(node_is_last(node));
        unsafe { (*node).set(PageListNode::TAKEN, false).set(PageListNode::LAST, false); }
    }

    fn has_contig_space(&self, start_node: *mut PageListNode, n: usize, i: usize) -> bool {
        // Now we look to see if we have
        // contiguous non-taken pages from this page
        for j in i..i + n {
            if node_is_taken(unsafe {start_node.add(j)}) {
                return false;
            }
        }
        true
    }

    fn node_for_address<T>(&self, p: *const T) -> *mut PageListNode {
        unsafe {
        (HEAP_START + (p as usize - self.alloc_start) / PAGE_SIZE) as *mut PageListNode
        }
    }

}

fn node_is_taken(node: *mut PageListNode) -> bool {
    unsafe { (*node).get(PageListNode::TAKEN) }
}

fn node_is_last(node: *mut PageListNode) -> bool {
    unsafe { (*node).get(PageListNode::LAST) }
}

fn zero_bytes<T>(p: *const T, b_cnt: usize) {
    unsafe {
        for i in p as usize..p as usize + b_cnt {
            (i as *mut u8).write_volatile(0);
        }
    }
}

fn page_count() -> usize {
    unsafe { crate::HEAP_SIZE / PAGE_SIZE }
}

/// Align a pointer to `2^order` bytes
pub fn align(ptr: usize, order: usize) -> usize {
    let o = (1 << order) - 1;
    (ptr + o) & !o
}
