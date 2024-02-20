#ifndef IO_H
#define IO_H
#include "types.h"

// ==================================================
// ===================== MMIO =======================
// ==================================================

void mmio_writeb(addr_t addr, int offset, byte_t val);
char mmio_readb(addr_t addr, int offset);

#endif 