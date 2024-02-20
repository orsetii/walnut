.option norvc
.altmacro
.set NUM_GP_REGS, 32  # Number of registers per context
.set REG_SIZE, 8   # Register size (in bytes)

# Use macros for saving and restoring multiple registers
.macro save_gp i, basereg=t6
	sd	x\i, ((\i)*REG_SIZE)(\basereg)
.endm
.macro load_gp i, basereg=t6
	ld	x\i, ((\i)*REG_SIZE)(\basereg)
.endm
.macro save_fp i, basereg=t6
	fsd	f\i, ((NUM_GP_REGS+(\i))*REG_SIZE)(\basereg)
.endm
.macro load_fp i, basereg=t6
	fld	f\i, ((NUM_GP_REGS+(\i))*REG_SIZE)(\basereg)
.endm

.section .text
.global m_trap_vector
# This must be aligned by 4 since the last two bits
# of the mtvec register do not contribute to the address
# of this vector.
.align 4
m_trap_vector:
	# All registers are volatile here, we need to save them
	# before we do anything.
	csrrw	t6, mscratch, t6

	.set 	i, 0
	.rept	31
		save_gp	%i
		.set	i, i+1
	.endr

	# Save the actual t6 register, which we swapped into
	# mscratch
	mv		t5, t6
	csrr	t6, mscratch
	save_gp 31, t5

	# Restore the kernel trap frame into mscratch
	csrw	mscratch, t5

	csrr	t1, mstatus
	srli	t0, t1, 13
	andi	t0, t0, 3
	li		t3, 3
	bne		t0, t3, post_fp_save
	# Save floating point registers
	.set 	i, 0
	.rept	32
		save_fp	%i, t5
		.set	i, i+1
	.endr
post_fp_save:
	# Get ready to go into Rust (trap.rs)
	# We don't want to write into the user's stack or whomever
	# messed with us here.
	# csrw	mie, zero

	csrr	a0, mepc
	sd		a0, 520(t5)
	csrr	a1, mtval
	csrr	a2, mcause
	csrr	a3, mhartid
	csrr	a4, mstatus
	csrr	a5, mscratch
	la		t0, KERNEL_STACK_END
	ld		sp, 0(t0)
	call	m_trap

	# When we get here, we've returned from m_trap, restore registers
	# and return.
	# m_trap will return the return address via a0.

	csrw	mepc, a0
	# Now load the trap frame back into t6
	csrr	t6, mscratch

	csrr	t1, mstatus
	srli	t0, t1, 13
	andi	t0, t0, 3
	li		t3, 3
	bne		t0, t3, restore_gp
	.set	i, 0
	.rept	32
		load_fp %i
		.set i, i+1
	.endr
restore_gp:
	# Restore all GP registers
	.set	i, 1
	.rept	31
		load_gp %i
		.set	i, i+1
	.endr

	# Since we ran this loop 31 times starting with i = 1,
	# the last one loaded t6 back to its original value.
	mret


