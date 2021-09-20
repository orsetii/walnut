use core::alloc::Layout;
use core::{marker::PhantomData, sync::atomic::AtomicPtr};
use core::sync::atomic::Ordering::SeqCst;
use spin::Mutex;
use lazy_static::lazy_static;
use if_chain::if_chain;

use crate::mm::{PhysAddr, Range, RangeSet, VirtAddr};

pub static PHYSICAL_ALLOCATOR: AtomicPtr<BumpFrameAllocator> = AtomicPtr::new(core::ptr::null_mut());

struct AllocatorInfo {
    frame_allocator: Mutex<Option<BumpFrameAllocator>>,
    free_frames: Mutex<Option<Vec<PhysAddr>>>,
}

lazy_static! {
    static ref ALLOCATOR_INFO: AllocatorInfo = AllocatorInfo {
        frame_allocator: Mutex::new(None),
        free_frames: Mutex::new(None),
    };
}

pub fn init_global_frame_alloc(frame_alloc: BumpFrameAllocator) {
    // set the frame allocator as our current allocator
    ALLOCATOR_INFO.frame_allocator.lock().replace(frame_alloc);
    let old_free_frames = ALLOCATOR_INFO.free_frames.lock().take();
    // avoid dropping this inside a lock so we don't trigger a free
    // while holding the lock
    drop(old_free_frames);
    ALLOCATOR_INFO
        .free_frames
        .lock()
        .replace(Vec::with_capacity(200));
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BumpFrameAllocator {
    start: usize,
    end: usize,
    next: usize,
    allocations: usize,
}

impl BumpFrameAllocator {
    const PAGE_SIZE: usize = 4 * 1024; // 4 KiB 
    pub fn new(r: Range) -> Self {
        Self {
            start: r.start as usize,
            end: r.end as usize,
            next: 0,
            allocations: 0,
        }
    }

    pub fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.next = start;
    }
    /// Attemps to allocate a page
    /// returning an `Option<T>` if successful or not
    pub fn alloc(&mut self) -> Option<PhysAddr> {
        // TODO check alignment and bounds check
        let page_base_addr = self.next;
        self.step_next()?;
        self.allocations += 1;
        Some(PhysAddr(page_base_addr))
    }

    pub fn dealloc(&mut self) -> Option<()> {
        self.allocations -= 1;
        if self.allocations == 0 {
            self.next = self.start;
        }
        Some(())
    }

    /// Increments the `next` value, bounds checked.
    fn step_next(&mut self) -> Option<()> {
        self.start.checked_add(Self::PAGE_SIZE)
                    .map(|_| Some(()))?
    }
}

/// Holds a standard 4KiB Page of Virtual Memory
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct Page(pub [u8; 4096]);


/// Holds a 4Kib of Physical Memory
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct PageFrame(pub [u8; 4096]);


#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct PageDirectory(pub u32);

pub fn init(mut range: RangeSet) {

    let frame_allocator = BumpFrameAllocator::new(range.largest());

    init_global_frame_alloc(frame_allocator);
}


