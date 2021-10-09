//! This is Walnut OS, a small operating system intended for use on embedded devices, currently
//! only officially supporting the Raspberry Pi 4.
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/orsetii/walnut/73f4d82ac61880ac1af22b9509e6ab5b9268f9d5/assets/img/WalnutComplete.svg"
)]
#![no_main]
#![no_std]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]

mod bsp;
mod console;
mod cpu;
mod panic_wait;
mod print;

/// Init the kernel
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    println!("[0] Hello from Rust!");

    panic!("Stopping...")
}


Quickly create and run optimised Windows, macOS and Linux desktop virtual machines.
