use core::arch::asm;

use crate::cpu::csr::ControlStatusRegister;

use self::mode::Mode;

pub mod csr;
pub mod mode;
pub mod port;
pub mod trap;
pub mod util;

/// Delete exceptions and interrupts to Supervisor mode
pub fn delegate_traps() {
    ControlStatusRegister::Medeleg.write(0xffff);
    ControlStatusRegister::Mideleg.write(0xffff);
    ControlStatusRegister::Sie.write(
        ControlStatusRegister::Sie.read() |
          1 << 9 // external interrupts
        | 1 << 5 // timer    interrupts
        | 1 << 1, // software interrupts
    );
}

/// Transitions into a given mode using the `xRET` instructions.
///
/// For example, transitioning from `Machine` to `Supervisor`
/// like we do in `kinit`, we perform an `mret` instruction.
///
/// # Safety
///
/// If we attempt this with incorrect Modes
/// it WILL panic.
///
/// If certain elements and CSRs are not set up
/// correctly, it WILL panic.
pub unsafe fn transition(dest_mode: Mode) -> ! {
    match (mode::Mode::current(), dest_mode) {
        (Mode::Machine, Mode::Supervisor) => {
            // Adjust the mode so we transition into S-mode
            mode::set_prev_privilege_mode(Mode::Supervisor);

            // Set the MEPC so that after we `mret`
            // we will be in `kmain`
            ControlStatusRegister::Mepc.write_fn_addr(crate::kmain);

            asm!("mret");
            unreachable!();
        }
        _ => unreachable!(),
    }
}

pub fn save_hartid() {
    ControlStatusRegister::ThreadPointer.write(ControlStatusRegister::Mhartid.read());
}
