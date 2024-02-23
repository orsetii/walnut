#![no_std]
#![no_main]

mod asm;
mod cpu;
mod drivers;
mod graphics;
mod init;
mod mem;
mod process;
mod sync;
mod util;

#[no_mangle]
extern "C" fn kmain() {
    panic!();
}
