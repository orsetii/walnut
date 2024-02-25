use core::ptr::null;
use block::{BlockPtr, Block};

use super::pages::{PAGE_ALLOCATOR, PAGE_SIZE};
use core::error::Error;

pub mod block;

/// How many pages we allocate on OS initialization
const INITIAL_KMEM_PAGE_COUNT: usize = 256;


pub type AllocResult<T> = core::result::Result<T, AllocationError>;

pub static mut ALLOCATOR: Allocator = Allocator { 
    block_cnt: 0,
    free_list_head: null()
};


#[derive(Debug)]
pub struct Allocator {
    block_cnt: usize,
    free_list_head: BlockPtr,
}

impl Allocator {
    /// Initialize Walnut's Allocator
    pub fn init(&mut self) -> AllocResult<()> {

        // allocate pages
        unsafe {
            self.free_list_head = PAGE_ALLOCATOR.zalloc(INITIAL_KMEM_PAGE_COUNT)
                                .ok_or(AllocationError::new ("Was not able to allocate 
                                pages for the kernel memory"))? as *const Block;
        }

        // Create the linked list to track out-of-use blocks
        self.block_cnt = (INITIAL_KMEM_PAGE_COUNT * PAGE_SIZE) / block::BLOCK_SIZE;

        for i in 0..self.block_cnt {
            unsafe {
                let b: &mut Block = Block::at_offset(self.free_list_head, i * block::BLOCK_SIZE);
                b.size = block::BLOCK_SIZE;
                b.next = self.free_list_head.add((i+1) * block::BLOCK_SIZE);
            }
        }

        unsafe {
            Block::at_offset(self.free_list_head,
                    (self.block_cnt -1) * block::BLOCK_SIZE).next = null();
        }
        
        Ok(())

    }

    pub fn block_alloc(&self) {
        todo!();
    }
    pub fn block_dealloc(&self) {
        todo!();
    }

    pub fn print_blocklist(&self) {
        let mut b = unsafe {Block::at_offset(self.free_list_head, 0) as BlockPtr };
            while unsafe { !(*b).next.is_null()} {
                crate::println!("{:#x?}", (*b));
                unsafe { 
                b = (*b).next;
                }
        }
    }
}


#[derive(Debug)]
pub struct AllocationError {
    details: &'static str,
}

impl AllocationError {
    pub fn new(msg: &'static str) -> AllocationError {
        AllocationError { details: msg }
    }
}

impl core::fmt::Display for AllocationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AllocationError {
    fn description(&self) -> &str {
        self.details
    }
}


