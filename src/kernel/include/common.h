#ifndef COMMON_H
#define COMMON_H
#include "common/assert.h"
#include "types.h"
#define __ASM_STR(x) #x

#define csr_swap(csr, val)                                      \
    ({                                                          \
        unsigned long __v = (unsigned long)(val);               \
        __asm__ __volatile__("csrrw %0, " __ASM_STR(csr) ", %1" \
                             : "=r"(__v)                        \
                             : "rK"(__v)                        \
                             : "memory");                       \
        __v;                                                    \
    })

#define csr_read(csr)                                   \
    ({                                                  \
        register unsigned long __v;                     \
        __asm__ __volatile__("csrr %0, " __ASM_STR(csr) \
                             : "=r"(__v)                \
                             :                          \
                             : "memory");               \
        __v;                                            \
    })

#define csr_write(csr, val)                                \
    ({                                                     \
        unsigned long __v = (unsigned long)(val);          \
        __asm__ __volatile__("csrw " __ASM_STR(csr) ", %0" \
                             :                             \
                             : "rK"(__v)                   \
                             : "memory");                  \
    })

#define READ_CSR(reg) ({ unsigned long __tmp; \
        asm volatile ("csrr %0, " #reg : "=r"(__tmp)); \
        __tmp; })

#define GETHARTID() READ_CSR(mhartid)

#endif
