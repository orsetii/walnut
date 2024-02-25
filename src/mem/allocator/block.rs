/// Size of each block, in bytes.
pub const BLOCK_SIZE: usize = 64;

pub type BlockPtr = *const Block;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub next: BlockPtr,
    pub size: usize
}

impl Block {
    /// Create a block, and fill its fields at a given offset from 
    /// the address of a head of a block list
    ///
    /// # Safety
    /// 
    pub unsafe fn at_offset(list_head: BlockPtr, offset: usize) -> &'static mut Block {
        &mut *(list_head as *mut Block).add(offset)
    }
}
