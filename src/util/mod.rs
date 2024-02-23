mod panic;

#[macro_export]
macro_rules! main_thread_only {
    ($block:block) => {
        if unsafe { $crate::cpu::util::my_hart()} == 0 { 
            $block
        }
    }
}
