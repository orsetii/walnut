#include "mem/addr.h"

paddr translate_virtual_address(vaddr va) {
  int vpn[3] = {(va.fields.vpn2 >> 27) & 0x1FF, (va.fields.vpn1 >> 18) & 0x1FF,
                (va.fields.vpn0 >> 9) & 0x1FF};

  ptable *current_table = get_root_page_table();

  for (int i = 2; i >= 0; i--) {
    ptable_entry pte = current_table->entries[vpn[i]];
    if (!is_valid(&pte))
      return PADDR(NULL); // Page fault

    if (is_leaf(&pte)) {
      return PADDR((pte.ppn << 12) |
                   (va.raw & 0xFFF)); // Construct physical address
    } else {
      current_table =
          (ptable *)((size_t)pte.ppn << 12); // Next page table level
    }
  }

  return PADDR(NULL); // Page fault
}
