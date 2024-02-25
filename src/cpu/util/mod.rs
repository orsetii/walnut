use super::csr::ControlStatusRegister;

/// Gets the HART ID of the running CPU
///
/// # Safety
///
/// If this is ran BEFORE the Hart ID is set,
/// or if `tp0` was cleared/changed, we will return
/// incorrect data.
pub unsafe fn my_hart() -> usize {
    ControlStatusRegister::ThreadPointer.read()
}
