##! Boot assembly code for macaque
##! 
##! ## Naming Conventions
##! All functions are prefixed with `_`
##!
##! All labels are prefixed with `_<function_name>_<mode_intial>_`
##! 					Example: `_start_s_return`
##! 
##!
##! 
##!
##! 
##!
##!
.option norvc
.section .data

.section .init
.global _start

_start:
	.cfi_startproc

		# Read the hart ID
		csrr t0, mhartid  
		# if not in hart #0, jump to 'wait for interrupt' loop
		bnez t0, hart_parking_lot 

_start_m_main_hart_thread_only:
		# Ensure SATP is zero
		csrw satp, zero

		.option push
		.option norelax
		la gp, __global_pointer
		.option pop

_start_m_validate_bss:
		# Sanity check the BSS section
		#
		# This is effectively an assert that the 
		# start address is before the end address
		la      a0,     __bss_start     
		la      a1,     __bss_end
		# skip zeroing if not needed
		bgeu    a0,     a1, _start_m_delegate_interrupts            

		# Loop through entire bss section, and zero it all
_start_m_bss_zero_loop:
				sd      zero,   (a0)
				addi    a0,     a0,         8
				bltu    a0,     a1,         _start_m_bss_zero_loop
				j _start_m_delegate_interrupts
		
_start_m_delegate_interrupts:
		# TODO delegate interrupts to be handled in s-mode

_start_m_init_stack:
		# load the stack pointer from
		# the link script. 	
		# It is calculated as _bss_end + 0x80000 (524 KiB Total)
		la sp, __kernel_stack_end

_start_m_kinit_init_mstatus:
		.set M_ENABLE_MACHINE_MODE, (0b11 << 11)
		li		t0, M_ENABLE_MACHINE_MODE
		csrw	mstatus, t0

# Load the `machine trap vector` *rust* function
# into `mtvec`. This function will now be called
# every time there is a trap. (syscall, illegal instruction, timer interupt, etc.)
_start_m_load_trap_vector:
		la t2, m_trap_vector
		csrw mtvec, t2

_start_m_disable_interrupts:
		csrw mie, zero


# Load the kinit function address 
# into the `Machine Exception Program Counter` CSR
# 
# Set return address to go into supervisor mode
#
# And jump to `kinit`
_start_m_m_kinit:
		la t1, kinit
		csrw mepc, t1

		la ra, _start_supervisor_mode_entry
		mret


# =========================================================================================
# ===================================== SUPERVISOR MODE  ==================================
# =========================================================================================

_start_supervisor_mode_entry:

_start_s_kmain_init_mstatus:
		.set M_ENABLE_SUPERVISOR_MODE, (1 << 11)
		.set M_ENABLE_PREV_MACHINE_INTERRUPT, (1 << 7)
		.set M_ENABLE_PREV_INTERRUPT, (1 << 5)
		li		t0, M_ENABLE_SUPERVISOR_MODE | M_ENABLE_PREV_MACHINE_INTERRUPT | M_ENABLE_PREV_INTERRUPT
		csrw	mstatus, t0


_start_s_init_stvec:
		la		t2, m_trap_vector
		csrw mtvec, t2



# Load the kmain function address 
# into the `Supervisor Exception Program Counter` CSR
# This is technically needed only when executing 
# a S-mode to U-mode change, which we are NOT 
# performing here (note the lack of `sret` below)
_start_s_load_kmain:
		la t1, kmain
		csrw mepc, t1

_start_s_init_mie:
	# TODO make the flags properly here
	# 1 << 1    : Supervisor software interrupt enable (SSIE=1 [Enabled])
	# 1 << 5    : Supervisor timer interrupt enable (STIE=1 [Enabled])
	# 1 << 9    : Supervisor external interrupt enable (SEIE=1 [Enabled])
	li		t2, 0x888
	csrw	mie, t2

_start_s_return:
		mret

# Note: i stole this code, i dont actually really know what or why it does. will revisit post-paging impl
hart_parking_lot:
    # Parked harts go here. We need to set these
	# to only awaken if it receives a software interrupt,
	# which we're going to call the SIPI (Software Intra-Processor Interrupt).
	# We call the SIPI by writing the software interrupt into the Core Local Interruptor (CLINT)
	# Which is calculated by: base_address + hart * 4
	# where base address is 0x0200_0000 (MMIO CLINT base address)
	# We only use additional harts to run user-space programs, although this may
	# change.

	# We divide up the stack so the harts aren't clobbering one another.
	la		sp, __kernel_stack_end
	li		t0, 0x10000
	csrr	a0, mhartid
	mul		t0, t0, a0
	sub		sp, sp, t0

    # The parked harts will be put into machine mode with interrupts enabled.
	li		t0, 0b11 << 11 | (1 << 7)
	csrw	mstatus, t0
	# Allow for MSIP (Software interrupt). We will write the MSIP from hart #0 to awaken these parked harts.
	li		t3, (1 << 3) | ~(1 << 5)
	csrw	mie, t3
    # Machine's exception program counter (MEPC) is set to the Rust initialization
	# code and waiting loop.
	la		t1, kinit_hart
	csrw	mepc, t1

    # Machine's trap vector base address is set to `m_trap_vector`, for
	# "machine" trap vector. The Rust initialization routines will give each
	# hart its own trap frame. We can use the same trap function and distinguish
	# between each hart by looking at the trap frame.
	la		t2, m_trap_vector
	csrw	mtvec, t2
    # Whenever our hart is done initializing, we want it to return to the waiting
	# loop, which is just below mret.
	la		ra, wfi_loop
	# We use mret here so that the mstatus register is properly updated.
	mret
	.cfi_endproc


wfi_loop:
		wfi
		j wfi_loop

