.section .rodata

    .global HEAP_START
HEAP_START:
    .dword __heap_start

    .global HEAP_SIZE
HEAP_SIZE:
    .dword __heap_size

    .global KERNEL_STACK_START
KERNEL_STACK_START:
    .dword __kernel_stack_start

    .global KERNEL_STACK_END
KERNEL_STACK_END:
    .dword __kernel_stack_end

    .global KERNEL_STACK_SIZE
KERNEL_STACK_SIZE:
    .dword __kernel_stack_size

    .global TEXT_START
TEXT_START:
    .dword stext
    .global TEXT_END
TEXT_END:
    .dword etext
