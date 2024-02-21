#ifndef MEM_H
#define MEM_H
#include "types.h"
#include "common.h"
#include "mem/addr.h"
#include "mem/plf.h"
#include "mem/page.h"
#include "mem/table.h"
#include "mem/allocator.h"

extern const size_t HEAP_START;
extern const size_t HEAP_SIZE;
extern const size_t TEXT_START;
extern const size_t TEXT_END;
extern const size_t DATA_START;
extern const size_t DATA_END;
extern const size_t RODATA_START;
extern const size_t RODATA_END;
extern const size_t BSS_START;
extern const size_t BSS_END;
extern const size_t KERNEL_STACK_START;
extern const size_t KERNEL_STACK_END;

void zero_bytes(void *ptr, size_t num_bytes);

#endif