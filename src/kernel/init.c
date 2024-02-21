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

    idmap_kheap();
}

void kinit_hart() {

}
