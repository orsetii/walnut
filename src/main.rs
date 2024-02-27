#![no_std]
#![no_main]
#![feature(asm_const, error_in_core)]

use core::arch::asm;

use crate::{cpu::csr::ControlStatusRegister, mem::{allocator::ALLOCATOR, pages}};
use alloc::{string::String, vec::Vec};
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

#[macro_use]
extern crate alloc;

#[no_mangle]
fn kmain() {
    main_thread_only!({
        info!("Welcome to Walnut!");
        // TODO handle this instead of unwrap
        if let Err(e) = main_hart_initialization() {
            panic!("Error initializing OS components in main hart: {}", e);
        }
    let s = String::from("Hello!");
    info!("Allocated string at {:p}: {}", &s,s);

    });



    hart_initialization();


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

fn hart_initialization() {

    ControlStatusRegister::Stvec.write(kernelvec as *const u8 as usize);
}
