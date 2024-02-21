#include "common.h"
#include "mem.h"
#include "print.h"
#include "serial.h"

void kmain() {

    init_ptable();


  kprintf("Entered main at: %p", kmain);
  while (1)
  {
  }
}
