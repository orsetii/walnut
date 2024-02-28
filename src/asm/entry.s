.section .bss
	.equ CPU_CNT, 4
.align 16
	stack0: .space 4096 * CPU_CNT

.section .text.init
.global _entry

# Entry point of the operating system
# All this does is load the stack pointer from
# where its calculated to be at compile-time (see kernel.ld)
# Each hart will run here.
_entry:
	# Index into the STACK0 byte array as
	# defined in `init/mod.rs`
	la sp, __kernel_stack_end 
        li a0, 1024*4
        csrr a1, mhartid
        addi a1, a1, 1
        mul a0, a0, a1
        sub sp, sp, a0
	call kinit

spin:
	j spin
