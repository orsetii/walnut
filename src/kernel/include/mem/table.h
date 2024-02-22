#ifndef TABLE_H
#define TABLE_H
#include "types.h"

#define PT_ENTRY_COUNT 512

#define PTE(x) CAST(ptable_entry, x)

/*
    A Page Table Entry.


    The permission bits, R, W, and X, indicate
    whether the page is readable, writable, and executable, respectively.
    When all three are zero, the PTE is a pointer to the next level of the page
   table; otherwise, it is a leaf PTE
*/
typedef union ptable_entry {

  struct {
    size_t V : 1;    // Valid bit
    size_t R : 1;    // Read permission
    size_t W : 1;    // Write permission
    size_t X : 1;    // Execute permission
    size_t U : 1;    // User-mode accessible
    size_t G : 1;    // Global mapping
    size_t A : 1;    // Accessed bit
    size_t D : 1;    // Dirty bit
    size_t RSW : 2;  // Reserved for software
    size_t ppn : 44; // Physical Page Number (only the used bits)
  };
  size_t bit_repr;
} ptable_entry;

typedef struct ptable {
  ptable_entry entries[PT_ENTRY_COUNT];
} ptable;

ptable *get_root_page_table();
void init_ptable();
bool is_valid(size_t ptable_entry);
bool is_branch(size_t ptable_entry);
bool is_leaf(size_t ptable_entry);

#endif
