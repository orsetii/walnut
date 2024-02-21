#ifndef ADDR_H
#define ADDR_H
#include "types.h"
#include "util.h"
#include "mem.h"

typedef union
{
    u64 raw;
    struct
    {
        u64 offset : 12; // Page offset
        u64 vpn0 : 9;    // VPN[0]
        u64 vpn1 : 9;    // VPN[1]
        u64 vpn2 : 9;    // VPN[2]
        u64 pad : 25;    // Padding to fill out the 64-bit space
    } fields;
} vaddr;

typedef union
{
    u64 raw;
    struct
    {
        u64 offset : 12; // Page offset, directly copied from the virtual address
        u64 ppn0 : 9;    // Least significant part of the PPN
        u64 ppn1 : 9;    // Middle part of the PPN
        u64 ppn2 : 26;   // Most significant part of the PPN, accommodating the larger physical address space
        u64 pad : 8;     // Padding to fill out the 64-bit space, as physical addresses are 56 bits
    } fields;
} paddr;

#define VADDR(x) CAST(vaddr, x)
#define PADDR(x) CAST(paddr, x)

paddr translate_virtual_address(vaddr va);

#endif