#include "common.h"
#include "io.h"
#include "mem.h"
#include "print.h"
#include "serial.h"

void kmain()
{
    kprintf("Entered main at: %p", kmain);
    initialize_uart();

    init_ptable();

    // kprintf("\nFound %x at 0x801f9008\n", *(u64*)0x801f9000);

    // mmio_writeb(mtimecmp, 0, mmio_readb(mtime, 0) + 10000000);
    //*(volatile char*)0x0 = 1;

    kprintf("Main Loop started...");
    while (1) {
    }
}
