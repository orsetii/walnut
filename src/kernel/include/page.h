#ifndef PAGE_H
#define PAGE_H
#include "types.h"


struct PageFlags {
    unsigned int TAKEN : 1;  // 1-bit flag for TAKEN status
    unsigned int LAST  : 1;  // 1-bit flag for LAST status
    unsigned int      : 6;  // 6 unused bits 
};


#endif