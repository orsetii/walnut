#![no_std]
#![no_main]
#![feature(asm_const, error_in_core)]

use core::arch::asm;

use crate::{cpu::csr::ControlStatusRegister, mem::{allocator::ALLOCATOR, pages}};
pub use util::Result;

pub mod asm;
pub mod cpu;
pub mod drivers;
pub mod graphics;
pub mod init;
pub mod mem;
pub mod process;
pub mod sync;
pub mod util;

extern "C" {
    static HEAP_SIZE: usize;
    static HEAP_START: usize;
fn kernelvec();
}

#[no_mangle]
fn kmain() {
    main_thread_only!({
        info!("Welcome to Walnut!");
        // TODO handle this instead of unwrap
        main_hart_initialization().unwrap();
    });


    ControlStatusRegister::Stvec.write(kernelvec as *const u8 as usize);

    unsafe {
        asm!("unimp");
    }


    loop {
        core::hint::spin_loop();
    }
}

fn main_hart_initialization() -> Result<()> {
    info!("We have a kernel heap size of {:#0x} ", HEAP_SIZE);
    unsafe {
        pages::PAGE_ALLOCATOR.init();
        ALLOCATOR.init()?;
    }
    Ok(())
}
