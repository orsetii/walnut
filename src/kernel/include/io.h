#ifndef IO_H
#define IO_H
#include "types.h"

// ==================================================
// ===================== MMIO =======================
// ==================================================

#define mtimecmp 0x02004000
#define mtime 0x0200bff8

void mmio_writeb(addr_t addr, int offset, byte_t val);
char mmio_readb(addr_t addr, int offset);

#endif
