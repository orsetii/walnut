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
.global s_trap_vector
.global m_trap_vector
# This must be aligned by 4 since the last two bits
# of the mtvec register do not contribute to the address
# of this vector.
.align 4
m_trap_vector:
# TODO save the registers below is an example impl

	# All registers are volatile here, we need to save them
	# before we do anything.
	#csrrw	t6, mscratch, t6
	# csrrw will atomically swap t6 into mscratch and the old
	# value of mscratch into t6. This is nice because we just
	# switched values and didn't destroy anything -- all atomically!
	# in cpu.rs we have a structure of:
	#  32 gp regs		0
	#  32 fp regs		256
	#  SATP register	512
	#  Trap stack       520
	#  CPU HARTID		528
	# We use t6 as the temporary register because it is the very
	# bottom register (x31)

# 	.set 	i, 1
# 	.rept	30
# 		save_gp	%i
# 		.set	i, i+1
# 	.endr
#
# 	# Save the actual t6 register, which we swapped into
# 	# mscratch
# save_gp:
# 	mv		t5, t6
# 	csrr	t6, mscratch
# 	save_gp 31, t5
#
# restore_trap_frame:
# 	# Restore the kernel trap frame into mscratch
# 	csrw	mscratch, t5
#
call_mtrap:
	csrr	a0, mepc
	csrr	a1, mtval
	csrr	a2, mcause
	csrr	a3, mhartid
	csrr	a4, mstatus
	call	m_trap

	# When we get here, we've returned from m_trap, restore registers
	# and return.
	# m_trap will return the return address via a0.

	csrw	mepc, a0

	# # Now load the trap frame back into t6
	# csrr	t6, mscratch

# restore_gp:
# 	# Restore all GP registers
# 	.set	i, 1
# 	.rept	31
# 		load_gp %i
# 		.set	i, i+1
# 	.endr

	# Since we ran this loop 31 times starting with i = 1,
	# the last one loaded t6 back to its original value.

	mret
# This must be aligned by 4 since the last two bits
# of the mtvec register do not contribute to the address
# of this vector.
.align 4
s_trap_vector:
# TODO save the registers below is an example impl

	# All registers are volatile here, we need to save them
	# before we do anything.
	#csrrw	t6, mscratch, t6
	# csrrw will atomically swap t6 into mscratch and the old
	# value of mscratch into t6. This is nice because we just
	# switched values and didn't destroy anything -- all atomically!
	# in cpu.rs we have a structure of:
	#  32 gp regs		0
	#  32 fp regs		256
	#  SATP register	512
	#  Trap stack       520
	#  CPU HARTID		528
	# We use t6 as the temporary register because it is the very
	# bottom register (x31)

# 	.set 	i, 1
# 	.rept	30
# 		save_gp	%i
# 		.set	i, i+1
# 	.endr
#
# 	# Save the actual t6 register, which we swapped into
# 	# mscratch
# save_gp:
# 	mv		t5, t6
# 	csrr	t6, mscratch
# 	save_gp 31, t5
#
# restore_trap_frame:
# 	# Restore the kernel trap frame into mscratch
# 	csrw	mscratch, t5
#
call_strap:
	csrr	a0, sepc
	csrr	a1, stval
	csrr	a2, scause
	csrr	a3, mhartid
	csrr	a4, sstatus
	call	s_trap

	# When we get here, we've returned from m_trap, restore registers
	# and return.
	# m_trap will return the return address via a0.

	csrw	sepc, a0

	# # Now load the trap frame back into t6
	# csrr	t6, mscratch

# restore_gp:
# 	# Restore all GP registers
# 	.set	i, 1
# 	.rept	31
# 		load_gp %i
# 		.set	i, i+1
# 	.endr

	# Since we ran this loop 31 times starting with i = 1,
	# the last one loaded t6 back to its original value.

	sret
