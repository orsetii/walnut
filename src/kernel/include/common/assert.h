#ifndef ASSERT_H
#define ASSERT_H

// TODO set mie to 0 here
#define HALT() ({        \
    asm volatile("wfi"); \
})

#define PANIC(format, ...) ({                  \
    kprintf("Kernel panic at %s:%d:\n" format, \
            __FILE__,                          \
            __LINE__                           \
                __VA_OPT__(, ) __VA_ARGS__);   \
    HALT();                                    \
})

#define ASSERT(condition, format, ...) ({             \
    if (!(condition))                                 \
        PANIC("Failed asserting that %s: " format,    \
              #condition __VA_OPT__(, ) __VA_ARGS__); \
})

#endif