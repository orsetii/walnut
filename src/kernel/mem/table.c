#include "mem.h"

static ptable* ROOT_PG_TABLE = NULL;

ptable* get_root_page_table() { return ROOT_PG_TABLE; }

/// Initializes the root page table.
///
/// This currently just amounts to
/// allocating a blank page.
/// As we do not *currently* perform any mapping
/// from machine->supervisor mode
void init_ptable()
{
    ptable* tp = pgalloc_zalloc(1);
    ROOT_PG_TABLE = tp;
}

/// @brief Is this Page Table Entry pointing to data, or just to another level
/// of the page table?
/// @param ptable_entry The Page Table Entry in question
/// @return Whether `ptable_entry` is a leaf or not (branch)
bool is_leaf(size_t pt_addr) { return !is_branch(pt_addr); }

/// @brief Is this Page Table Entry just pointing to another level of the page
/// table?
/// @param ptable_entry The Page Table Entry in question
/// @return Whether `ptable_entry` is a branch or not (leaf)
bool is_branch(size_t pt_addr)
{
    ptable_entry* pte = (ptable_entry*)pt_addr;
    return pte->R == 0 && pte->W == 0 && pte->X == 0;
}

/// @brief Is this Page Table Entry a `valid` entry?
/// @param ptable_entry The Page Table Entry in question
/// @return Whether this PTE is 'valid' or not. If not, the PTE should be
/// ignored and not used under any circumstances
bool is_valid(size_t pt_addr) { return ((ptable_entry*)pt_addr)->V; }
