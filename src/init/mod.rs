//! Initialization routines and structures to set up the operating system
//! this contains the first rust code that is executed by Walnut.

#[macro_use]
pub mod log;

use crate::cpu::{
    csr::ControlStatusRegister, delegate_traps, mode::Mode, save_hartid, transition, util::my_hart,
};

#[no_mangle]
extern "C" fn kinit() -> ! {
    save_hartid();
    info!("Initializing Hardware Thread {}", my_hart());

    // Disable paging (for now)
    ControlStatusRegister::Satp.write(0);

    // Delegate interrupts
    delegate_traps();

    // configure PMP (Physical Memory Protection)
    // so supervisor mode can access all of physical memory

    ControlStatusRegister::Pmpaddr0.write(0x3fffffffffffff);
    ControlStatusRegister::Pmpcfg0.write(0xf);

    // TODO: ask for interrupts
    // TODO: why does xv6 keep hartid in tp reg for cpuid?

    unsafe { transition(Mode::Supervisor) }
}
