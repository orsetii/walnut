#ifndef ALLOCATOR_H
#define ALLOCATOR_H

#include "mem.h"

#define BLOCK_SIZE 64
#define INTIAL_KMEM_PAGE_CNT 32

#define PAGES_FOR_BLOCK_CNT(size) \
    (((size) + BLOCK_SIZE - 1) / BLOCK_SIZE + PAGE_SIZE - 1) / PAGE_SIZE

#define BLOCKS_FOR_BYTE_CNT(byte_count) \
    ((byte_count + (BLOCK_SIZE - 1)) / BLOCK_SIZE)

typedef struct block_t block;

typedef struct block_t
{
    /// @brief The next free block
    block *next;
    /// @brief Size of this `block` (in `block`s)
    size_t size;
} block;

size_t kmem_head();
void idmap_kheap();
void allocator_init();

void *kmalloc(size_t sz);
void kfree(void *p);

void block_free(void *b);
void *block_alloc(size_t block_cnt);

size_t align(size_t n);

#endif
