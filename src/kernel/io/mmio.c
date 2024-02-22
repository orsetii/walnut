#include "io.h"
void mmio_writeb(addr_t addr, int offset, byte_t val) {
  *(char *)(addr + offset) = val;
}

char mmio_readb(addr_t addr, int offset) { return *(char *)(addr + offset); }
