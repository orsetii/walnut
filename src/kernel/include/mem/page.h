#include "common.h"
#include "mem.h"
#include "print.h"
#include "types.h"
#include "plf.h"

#ifndef PAGE_H
#define PAGE_H

#define NUM_OF_PAGES (HEAP_SIZE / PAGE_SIZE)

typedef unsigned char page_list_flags;
extern const size_t PAGE_SIZE;
extern const size_t PAGE_ORDER;

#define PLF_EMPTY 0
#define PLF_TAKEN 1
#define PLF_LAST 1 << 1

void pgalloc_init();
void *pgalloc_alloc(unsigned int pages);
void *pgalloc_zalloc(unsigned int pages);
void pgalloc_free(void *ptr);
page_list_flags *get_plf_for_addr(void *ptr);

#endif