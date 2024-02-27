//! Defines methods for accessing various Control Status Registers
//! on our RISC-V OS.
//!
//! For CSRs that are defined across modes, like `mstatus` and `sstatus` these are defined
//! in one module, and determining which one is used/acessed is determined at runtime.
//!

pub mod epc;
pub mod satp;
pub mod status;

pub enum ControlStatusRegister {
    /// Machine Exception Program Counter
    Mepc,

    /// Supervisor Exception Program Counter
    Sepc,

    /// Supervisor Address Translation and Protection
    Satp,

    /// Hardware Thread ID
    Mhartid,

    /// Machine Exception Delegation
    Medeleg,

    /// Machine Interrupt Delegation
    Mideleg,

    /// Supervisor Interrupt Enable
    Sie,

    /// Physical Memory Protection address
    Pmpaddr0,

    /// Physical Memory Protection Configuration
    Pmpcfg0,

    ///  Supervisor Trap Vector Base Address
    Stvec,

    /// Supervisor Trap Value
    Stval,

    /// Supervisor Cause
    Scause,

    /// Supervisor Status
    SStatus,

    /// Thread Pointer
    /// NOTE: this is not actually a CSR, but we currently
    /// mostly use it like one, so its here.
    ThreadPointer,
}

impl ControlStatusRegister {
    pub fn read(&self) -> usize {
        // We have to do this in a very stupid looking way,
        // because of strings etc, and we operate these functions from pre-alloc state.
        let result: usize;
        unsafe {
            match self {
                Self::Mepc => core::arch::asm!("csrr {0}, mepc", out(reg) result),
                Self::Sepc => core::arch::asm!("csrr {0}, sepc", out(reg) result),
                Self::Satp => core::arch::asm!("csrr {0}, satp", out(reg) result),
                Self::Medeleg => core::arch::asm!("csrr {0}, medeleg", out(reg) result),
                Self::Mideleg => core::arch::asm!("csrr {0}, mideleg", out(reg) result),
                Self::Sie => core::arch::asm!("csrr {0}, sie", out(reg) result),
                Self::Pmpaddr0 => core::arch::asm!("csrr {0}, pmpaddr0", out(reg) result),
                Self::Pmpcfg0 => core::arch::asm!("csrr {0}, pmpcfg0", out(reg) result),
                Self::Stvec => core::arch::asm!("csrr {0}, stvec", out(reg) result),
                Self::Stval => core::arch::asm!("csrr {0}, stval", out(reg) result),
                Self::Scause => core::arch::asm!("csrr {0}, scause", out(reg) result),
                Self::SStatus => core::arch::asm!("csrr {0}, sstatus", out(reg) result),
                Self::Mhartid => core::arch::asm!("csrr {0}, mhartid", out(reg) result),
                Self::ThreadPointer => core::arch::asm!("mv {0}, tp", out(reg) result),
            }
        }
        result
    }
    pub fn write(&self, v: usize) {
        // We have to do this in a very stupid looking way,
        // because of strings etc, and we operate these functions from pre-alloc state.
        unsafe {
            match self {
                Self::Mepc => core::arch::asm!("csrw mepc, {}", in(reg) v),
                Self::Sepc => core::arch::asm!("csrw sepc, {}", in(reg) v),
                Self::Satp => core::arch::asm!("csrw satp, {}", in(reg) v),
                Self::Medeleg => core::arch::asm!("csrw medeleg, {}", in(reg) v),
                Self::Mideleg => core::arch::asm!("csrw mideleg, {}", in(reg) v),
                Self::Sie => core::arch::asm!("csrw sie, {}", in(reg) v),
                Self::Pmpaddr0 => core::arch::asm!("csrw  pmpaddr0, {}", in(reg) v),
                Self::Pmpcfg0 => core::arch::asm!("csrw  pmpcfg0, {}", in(reg) v),
                Self::Stvec => core::arch::asm!("csrw  stvec, {}", in(reg) v),
                Self::Stval => core::arch::asm!("csrw  stval, {}", in(reg) v),
                Self::Scause => core::arch::asm!("csrw  scause, {}", in(reg) v),
                Self::SStatus => core::arch::asm!("csrw  sstatus, {}", in(reg) v),
                Self::Mhartid => core::arch::asm!("csrw  mhartid, {}", in(reg) v),
                Self::ThreadPointer => core::arch::asm!("mv  tp, {}", in(reg) v),
            }
        }
    }

    pub fn write_fn_addr(&self, f: fn()) {
        self.write(f as *const () as usize);
    }
    pub fn write_unsafe_fn_addr(&self, f: unsafe fn()) {
        self.write(f as *const () as usize);
    }
}
