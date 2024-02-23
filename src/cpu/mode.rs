use core::arch::asm;

/// The the current privilege mode this
/// CPU/HART is running in.
///
/// We know that we enter rust in machine mode (M-mode)
/// so it starts as such
static mut CURRENT_MODE: Mode = Mode::Machine;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// When running in a hypervisor context
    /// Unused in Walnut, and disregarded in documentation etc.
    Hypervisor,

    /// The top-level privilege mode, full access to physical memory is available
    /// In Walnut, we exit M-mode as soon as possible, to maximize safety.
    Machine,

    /// The privilege mode the kernel runs in
    Supervisor,

    /// Mode context for userspace processes
    User,
}

impl Mode {
    /// Get the current privilege mode this
    /// CPU/HART is running in.
    ///
    /// This is slightly trickier than you would imagine,
    /// as RISC-V deliberately doesn’t make it easy
    /// for code to discover what mode it is running
    /// it because this could be used for processes to discover they running from
    /// within a hypvervisor
    ///
    /// We could implement this by doing some trickery with
    /// checking previous privilige mode and perform some logic
    /// to retreive this, however this may become error-prone.
    /// So, we just track it in a global static mutable variable,
    /// changed upon mode-switch.
    ///
    /// This function's primary purpose is to provide a 'safe'
    /// API to grab the current mode.
    pub fn current() -> Mode {
        unsafe { CURRENT_MODE }
    }

    pub fn set_current(m: Mode) {
        unsafe {
            CURRENT_MODE = m;
        }
    }

    fn mpp_val(&self) -> usize {
        match self {
            Self::Machine => 3 << 11,
            Self::Supervisor => 1 << 11,
            Self::User => 0 << 11,
            Self::Hypervisor => unreachable!(),
        }
    }
}

impl From<usize> for Mode {
    fn from(value: usize) -> Self {
        const M: usize = 3 << 11;
        const S: usize = 1 << 11;
        const U: usize = 0 << 11;
        match value {
            M => Self::Machine,
            S => Self::Supervisor,
            U => Self::User,
            _ => unreachable!("Invalid value found for Mode!"),
        }
    }
}

pub fn set_prev_privilege_mode(m: Mode) {
    const MPP_MASK: usize = 3 << 11;
    let mut csr_data: usize;

    unsafe {
        asm!("csrr {}, mstatus", out(reg) csr_data);
    }

    // Mask off the previous privilege field
    // and set the supplied mode.
    csr_data &= !MPP_MASK;
    csr_data |= m.mpp_val();

    unsafe {
        asm!("csrw mstatus, {}", in(reg) csr_data);
    }

    assert!(m == get_prev_privilege_mode());
}

pub fn get_prev_privilege_mode() -> Mode {
    const MPP_MASK: usize = 3 << 11;
    let mut csr_data: usize;

    unsafe {
        asm!("csrr {}, mstatus", out(reg) csr_data);
    }

    Mode::from(csr_data & MPP_MASK)
}
