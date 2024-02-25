mod panic;
pub mod error;

pub type Result<T> = core::result::Result<T, error::WalnutError>;

/// Runs a given code block ONLY in hardware thread 0
#[macro_export]
macro_rules! main_thread_only {
    ($block:block) => {
        if unsafe { $crate::cpu::util::my_hart() } == 0 {
            $block
        }
    };
}
