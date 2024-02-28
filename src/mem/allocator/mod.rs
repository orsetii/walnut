use core::{alloc::GlobalAlloc, cell::UnsafeCell, ptr::{null, null_mut}};
use block::{BlockPtr, Block};

use crate::{debug, error, info, mem::allocator::block::{BlockPtrMut, BLOCK_SIZE}};

use super::pages::{PAGE_ALLOCATOR, PAGE_SIZE};
use core::error::Error;

pub mod block;

/// How many pages we allocate on OS initialization
const INITIAL_KMEM_PAGE_COUNT: usize = 256;


pub type AllocResult<T> = core::result::Result<T, AllocationError>;

#[global_allocator]
pub static mut ALLOCATOR: AllocGuard = AllocGuard { 
    allocator: UnsafeCell::new(Allocator {
    block_cnt: 0,
    free_list_head: null()
    })
};


#[derive(Debug)]
pub struct Allocator {
    pub block_cnt: usize,
    pub free_list_head: BlockPtr,
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

    pub fn sub_block_alloc(&mut self, byte_cnt: usize) -> AllocResult<*const u8> {
        self.block_alloc(Self::blocks_for_byte_sz(Self::align(byte_cnt)))
    }

    pub fn sub_block_dealloc(&mut self, p: *mut u8)  {
        self.block_dealloc(p)
    }

    pub fn block_alloc(&mut self, n: usize) -> AllocResult<*const u8> {
        if self.free_list_head.is_null() {
            error!("No available blocks. Unable to allocate!");
            return Err(AllocationError::new("No non-taken blocks found to allocate with"));
        }


        let mut current = self.free_list_head;
        let mut prev = null_mut::<Block>();

        unsafe {

        while !current.is_null() && (*current).size < n {
            prev = current as BlockPtrMut;
            current = (*current).next;
        }
        }


        if current.is_null() {
            
            debug!("No blocks big enough to hold {:#0x} found. Allocating {} pages.", n, Self::pages_for_block_cnt(n));
            unsafe {
            return Ok(PAGE_ALLOCATOR.alloc(Self::pages_for_block_cnt(n)).expect("Unable to allocate pages for alloc!") as *const u8);
            }
        }

        unsafe {

        if (*current).size > n {

                info!("Block found was too large. Splitting into blocks of {} and {}", (*current).size - n, n);
                let leftover_block = current.byte_add(n * BLOCK_SIZE) as BlockPtrMut;
                (*leftover_block).size = (*current).size - n;
                (*leftover_block).next = (*current).next;

                (*(current as BlockPtrMut)).size = n;

                // Make the new split block that we 
                // wont use available to the list
                // depending on how we found `current`
                if !prev.is_null() {
                    // this also removes `current`
                    // from the free list
                    (*prev).next = leftover_block as BlockPtr;
                } else {
                    self.free_list_head = leftover_block;
                }

        } else {
              if !prev.is_null() {
                    (*prev).next = (*current).next;
                } else {
                    self.free_list_head = (*current).next;

                }
            }

        }
        unsafe {
            assert!(current != (*current).next);
            assert!((*current).size > 0);
        }
        Ok(current as *const u8)
    }



    pub fn block_dealloc<T>(&mut self, ptr: *const T ) {

        let fb = ptr as BlockPtrMut;

        let mut current = self.free_list_head as BlockPtrMut;
        let mut prev = null_mut::<Block>();

        unsafe {
            while !current.is_null() && current < fb {
                prev = current;
                current = (*current).next as BlockPtrMut;
            }

            if !prev.is_null() && (prev.byte_add((*prev).size * BLOCK_SIZE)) as BlockPtr == fb {

                info!("Coalescing backwards from {:#0p} -> {:#0p}", prev, (*prev).next);
            } else {
                (*fb).next = self.free_list_head;
                self.free_list_head = fb as BlockPtr;
            }

            current = fb;

            while !current.is_null() && current != (*current).next as BlockPtrMut && (current.byte_add((*current).size * BLOCK_SIZE)) == (*current).next as BlockPtrMut {
                assert!((*current).size != 0);
                (*current).size += (*(*current).next).size;
                (*current).next = (*(*current).next).next;
                info!("Coalescing forwards from {:#0p} -> {:#0p}", current, (*current).next);
            }
        }
        info!("Freed block(s) at {:0p}", ptr);

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

    fn pages_for_block_cnt(n: usize) -> usize {
        // Calculate the number of blocks required
        let blocks = (n + BLOCK_SIZE - 1) / BLOCK_SIZE;

        // Calculate the number of pages required, rounding up
        (blocks + PAGE_SIZE - 1) / PAGE_SIZE
    }

    fn blocks_for_byte_sz(n: usize) -> usize {

        (n + (BLOCK_SIZE - 1)) / BLOCK_SIZE
    }
    fn align(n: usize) -> usize {
    (n + core::mem::size_of::<usize>() - 1) & !(core::mem::size_of::<usize>() - 1)
}
}

pub struct AllocGuard {
    allocator: UnsafeCell<Allocator>
}

impl AllocGuard {
    pub fn init(&self) -> AllocResult<()> {
        unsafe {
            (&mut *self.allocator.get()).init()
        }
    }
    pub fn alloc_cnt(&self) -> usize {
        unsafe {
            (&mut *self.allocator.get()).block_cnt
        }
    }
}

unsafe impl GlobalAlloc for AllocGuard {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        (&mut *self.allocator.get()).sub_block_alloc(layout.size()).unwrap() as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        (&mut *self.allocator.get()).sub_block_dealloc(ptr);
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


