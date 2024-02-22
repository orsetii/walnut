#include "mem.h"

/// @brief How many blocks we have total
static size_t NUM_BLOCKS = 0;

/// @brief The head of the 'free' list
block* KMEM_FLIST_HEAD;

size_t kmem_head() { return (size_t)KMEM_FLIST_HEAD; }

void allocator_init()
{
    // Allocate 64 pages
    void* kmem_alloc = pgalloc_zalloc(INTIAL_KMEM_PAGE_CNT);
    ASSERT(kmem_alloc != NULL,
        "allocator_init(): Unable to allocate pages for kernel memory");
    KMEM_FLIST_HEAD = kmem_alloc;

    NUM_BLOCKS = (INTIAL_KMEM_PAGE_CNT * PAGE_SIZE) / BLOCK_SIZE;

    for (size_t i = 0; i < NUM_BLOCKS; i++) {
        block* current = (block*)(KMEM_FLIST_HEAD + (i * BLOCK_SIZE));
        current->next = (block*)(KMEM_FLIST_HEAD + ((i + 1) * BLOCK_SIZE));
        current->size = BLOCK_SIZE;
    }
    // Set the last block's `next` to NULL, as we have no more space
    ((block*)(KMEM_FLIST_HEAD + (NUM_BLOCKS - 1) * BLOCK_SIZE))->next = NULL;
}

void* kmalloc(size_t sz) { return block_alloc(BLOCKS_FOR_BYTE_CNT(align(sz))); }

void kfree(void* p) { block_free(p); }

void* block_alloc(size_t block_cnt)
{
    if (KMEM_FLIST_HEAD == NULL) {
        kprintf("Ran out of memory, unable to allocate %d blocks\n", block_cnt);
        return NULL;
    }

    block* current = KMEM_FLIST_HEAD;
    block* prev = NULL;

    // Iterate until we find a block big enough
    while (current != NULL && current->size < block_cnt) {
        prev = current;
        current = current->next;
    }

    // If there were no big enough blocks, we allocate pages directly
    // this allocation will be merged back into and split anyway later.
    if (current == NULL) {
        kprintf("No blocks big enough to hold 0x%x bytes. Allocating %d pages "
                "instead. \n",
            block_cnt, PAGES_FOR_BLOCK_CNT(block_cnt));
        return pgalloc_alloc(PAGES_FOR_BLOCK_CNT(block_cnt));
    }

    if (current->size > block_cnt) {
        kprintf("Found a block that was too large. Splitting into two blocks of "
                "size %d and %d\n",
            current->size - block_cnt, block_cnt);
        // Create a new block, that uses the space we don't need
        // since `current` is too big.
        block* leftover_block = (block*)((char*)current + (block_cnt * BLOCK_SIZE));
        leftover_block->size = current->size - block_cnt;
        leftover_block->next = current;
        // And adjust currents size to fit
        current->size = block_cnt;

        if (prev) {
            prev->next = leftover_block;
        } else {
            KMEM_FLIST_HEAD = leftover_block;
        }
    } else {
        // If we are *not* splitting in this allocation
        // then we want to remove from the free list
        if (prev) {
            prev->next = current->next;
        } else {
            KMEM_FLIST_HEAD = current->next;
        }
    }

    ASSERT(current != current->next, "CURRENT WAS SAME AS CURRENT->NEXT");
    ASSERT(current->size > 0, "CURRENT HAS SIZE OF ZERO");
    return (void*)current;
}

/// @brief Frees the block at the given pointer, and coalesces into other blocks
/// if possible.
/// @param b The pointer to the block/data
void block_free(void* b)
{
    block* fb = ((block*)b);

    // Check if we can coalesce the previous block
    block* current = KMEM_FLIST_HEAD;
    block* prev = NULL;
    while (current != NULL && current < fb) {
        prev = current;
        current = current->next;
    }

    // Check at the boundary of `prev` if there is a free block
    if (prev && (prev + prev->size * BLOCK_SIZE) == fb) {
        prev->size += fb->size;
        prev->next = fb->next;
        kprintf("Coalescing backwards from 0x%x -> 0x%x\n", prev, prev->next);
    } else {
        fb->next = KMEM_FLIST_HEAD;
        KMEM_FLIST_HEAD = fb;
    }

    // Now, we check forwards from `fb`
    // and coalesce into the next block also.
    current = fb;
    while (current->next != NULL && current->next != current && (current + current->size * BLOCK_SIZE) == current->next) {
        ASSERT(current->size == 0, "CURRENT HAS SIZE OF ZERO");
        // Coalesce with next
        current->size += current->next->size;
        current->next = current->next->next;
        kprintf("Coalescing forwards from 0x%x -> 0x%x\n", current, current->next);
    }
}

// TODO: Wordsize should be defined by user upon compiling walnut
//       for now, we define it here like lazy bums
#define __WORDSIZE 64

/// @brief Aligns a byte size variable upwards so it is optimized for memory
/// access
/// @param n Requested byte size
/// @return Optimal byte count
inline size_t align(size_t n)
{
    return (n + __WORDSIZE - 1) & ~(__WORDSIZE - 1);
}
