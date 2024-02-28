.section .rodata

    .global HEAP_START
HEAP_START:
    .dword __heap_start

    .global HEAP_SIZE
HEAP_SIZE:
    .dword __heap_size

    .global TEXT_START
TEXT_START:
    .dword stext
    .global TEXT_END
TEXT_END:
    .dword etext
