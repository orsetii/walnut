#include "mem.h"

static ptable *ROOT_PG_TABLE = NULL;

ptable *get_root_page_table()
{
    return ROOT_PG_TABLE;
}

void init_ptable()
{
    ptable *tp = pgalloc_zalloc(1);
    ROOT_PG_TABLE = tp;
    set_satp(0);
}

void set_satp(u64 root_page_table_ppn)
{
    u64 satp_value = (8 << 60) | (root_page_table_ppn);
    csr_write(0x180, satp_value);
}

void map(size_t v_addr, size_t p_addr, u64 bits, int level)
{
    ASSERT(ROOT_PG_TABLE != NULL, "Root page table was NULL");
    ptable *current_table = get_root_page_table();
    vaddr va = VADDR(v_addr);
    paddr pa = PADDR(p_addr);

    size_t vpn[3] = {
        // VPN[0] = vaddr[20:12]
        (v_addr >> 12) & 0x1ff,
        // VPN[1] = vaddr[29:21]
        (v_addr >> 21) & 0x1ff,
        // VPN[2] = vaddr[38:30]
        (v_addr >> 30) & 0x1ff,
    };
    const size_t ppn[3] = {
        // PPN[0] = paddr[20:12]
        (p_addr >> 12) & 0x1FF,

        // PPN[1] = paddr[29:21]
        (p_addr >> 21) & 0x1FF,

        // PPN[2] = paddr[55:30]
        (p_addr >> 30) & 0x3FFFFFF};

    u64 *pte = &current_table->entries[vpn[2]];

    for (int i = 1; i >= level; --i)
    {
        // TODO FIXME: We are currently erroring on this deref
        // but im too tired
        if (!is_valid(*pte))
        {
            void *page = pgalloc_alloc(1);
            *pte = ((u64)page >> 2) | PTE_VALID;
        }
        uint64_t *entry = (uint64_t *)((*pte & ~0x3FFull) << 2);
        pte = &entry[vpn[i]];
    }

    *pte = (ppn[2] << 28) | // ppn[2] = PTE[53:28]
           (ppn[1] << 19) | // ppn[1] = PTE[27:19]
           (ppn[0] << 10) | // ppn[0] = PTE[18:10]
           bits | PTE_VALID;
}

void unmap(size_t v_addr)
{
    ptable *current_table = get_root_page_table();
    ASSERT(current_table != NULL, "unmap(): root should not be NULL");

    // Start with level 2
    for (size_t lv2 = 0; lv2 < PT_ENTRY_COUNT; ++lv2)
    {
        ptable_entry entry_lv2 = current_table->entries[lv2];
        if (is_valid(entry_lv2.bit_repr) && is_branch(entry_lv2.bit_repr))
        {
            // This is a valid entry, so drill down and free
            ptable *table_lv1 = (ptable *)((entry_lv2.bit_repr & ~0x3FFull) << 2);
            // Now repeat the process with level 1
            for (size_t lv1 = 0; lv1 < PT_ENTRY_COUNT; ++lv1)
            {
                ptable_entry entry_lv1 = table_lv1->entries[lv1];
                if (is_valid(entry_lv1.bit_repr) && is_branch(entry_lv1.bit_repr))
                    // We can't have branches in level 0, so free directly
                    pgalloc_free((void *)((entry_lv1.bit_repr & ~0x3FFull) << 2));
            }
            pgalloc_free((void *)table_lv1);
        }
    }
}

void id_map_range(size_t start, size_t end, u64 bits)
{
    ASSERT(start <= end, "id_map_range(): start must not exceed end");
    ASSERT(is_leaf(&bits),
           "id_map_range(): Provided bits must correspond to leaf entry");
    size_t memaddr = start & ~(PAGE_SIZE - 1);
    size_t num_kb_pages = (align_val(end, PAGE_ORDER) - memaddr) / PAGE_SIZE;
    for (size_t i = 0; i < num_kb_pages; ++i)
    {
        map(memaddr, memaddr, bits, 0);
        memaddr += PAGE_SIZE;
    }
}

/// @brief Is this Page Table Entry pointing to data, or just to another level of the page table?
/// @param ptable_entry The Page Table Entry in question
/// @return Whether `ptable_entry` is a leaf or not (branch)
bool is_leaf(size_t pt_addr)
{
    return !is_branch(pt_addr);
}

/// @brief Is this Page Table Entry just pointing to another level of the page table?
/// @param ptable_entry The Page Table Entry in question
/// @return Whether `ptable_entry` is a branch or not (leaf)
bool is_branch(size_t pt_addr)
{
    ptable_entry *pte = (ptable_entry *)pt_addr;
    return pte->R == 0 &&
           pte->W == 0 &&
           pte->X == 0;
}

/// @brief Is this Page Table Entry a `valid` entry?
/// @param ptable_entry The Page Table Entry in question
/// @return Whether this PTE is 'valid' or not. If not, the PTE should be ignored and not used under any circumstances
bool is_valid(size_t pt_addr)
{
    return ((ptable_entry *)pt_addr)->V;
}