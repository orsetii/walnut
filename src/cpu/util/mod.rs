use core::arch::asm;


pub fn my_hart() -> usize {
    let id: usize;
    unsafe {
        asm!("csrr {}, mhartid", out(reg) id);
    }
    id
}
