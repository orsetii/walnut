#include "serial.h"
#include "print.h"
#include "common.h"
#include "mem.h"

void kinit() {
    kprintln("Walnut initializing...");
    initialize_uart();
    pgalloc_init();
    init_ptable();
    allocator_init();
    //*(volatile u64*)0x801f9000 = 0xdeadbeef;
}

void kinit_hart() {

}
