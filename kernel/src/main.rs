#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(lang_items)]

mod io;
mod lang;

use bootloader::{entry_point, BootInfo};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {

    io::serial::init();

    println!("{:#x?}", boot_info);




    panic!("END OF KERNEL MAIN");
}
