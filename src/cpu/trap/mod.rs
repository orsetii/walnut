use crate::debug;

use super::csr::ControlStatusRegister;

#[derive(Debug)]
pub enum Interrupt {
    Software,
    Timer,
    External,
    Reserved(usize),
    PlatformUse(usize),
}

impl From<usize> for Interrupt {
    fn from(value: usize) -> Self {
        let code = value & !(1 << 63);
        match code {
            1 => Self::Software,
            5 => Self::Timer,
            9 => Self::External,
            0 | 2..=4 | 6..=8 | 10..=15 => Self::Reserved(code),
            16.. => Self::PlatformUse(code),
        }
    }
    
}

#[derive(Debug)]
pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
}

impl From<usize> for Exception {
    fn from(value: usize) -> Self {
        // the msb is not set for exceptions so 
        // dont need to remove it.
        match value {
            0 => Self::InstructionAddressMisaligned,
            1 => Self::InstructionAccessFault,
            2 => Self::IllegalInstruction,
            _ => todo!(),
        }
    }
}



#[no_mangle]
extern "C" fn handle_trap() 
{
    let sepc = ControlStatusRegister::Sepc.read();
    let status = ControlStatusRegister::SStatus.read(); 
    let scause = ControlStatusRegister::Scause.read(); 
    let trap_val = ControlStatusRegister::Scause.read(); 
    crate::info!("SEPC={:#0x} SSTATUS={:#0x} SCAUSE={:#0x} STVAL={:#0x} ", sepc, status, scause, trap_val);

    if is_interrupt(scause) {
        handle_interrupt()
    } else {
        handle_exception()
    }
}

fn handle_interrupt() {
    let scause = ControlStatusRegister::Scause.read(); 

    debug!("Interrupt: {:?}", Interrupt::from(scause));
}
fn handle_exception() {
    use Exception::*;

    let scause = ControlStatusRegister::Scause.read(); 

    let exception = Exception::from(scause);    
    debug!("Exception: {:?}", exception);

    match exception {
        IllegalInstruction => {
            panic!("ILLEGAL INSTRUCTION DETECTED");
        },
        _ => {},
    }

    // Increment the SEPC
    ControlStatusRegister::Sepc.write(ControlStatusRegister::Sepc.read()+4);
}



fn is_interrupt(v: usize) -> bool {
    v >> 63 == 1
}

fn is_exception(v: usize) -> bool {
    !is_interrupt(v)
}
