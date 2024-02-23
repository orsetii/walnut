use core::arch::global_asm;

global_asm!(include_str!("entry.s"));
global_asm!(include_str!("trap.s"));
