use super::csr::ControlStatusRegister;




#[no_mangle]
extern "C" fn handle_trap() 
{
    let sepc = ControlStatusRegister::Sepc.read();
    crate::info!("SEPC is: {:#0x}", sepc);
    panic!();
}
