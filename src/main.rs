#![no_std]
#![no_main]
#![feature(asm_const)]

pub mod asm;
pub mod cpu;
pub mod drivers;
pub mod graphics;
pub mod init;
pub mod mem;
pub mod process;
pub mod sync;
pub mod util;

#[no_mangle]
fn kmain() {
    main_thread_only!({
        info!("Welcome to Walnut!");
    });
    loop {}
}
