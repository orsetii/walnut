pub mod cpu;
pub mod gosling;
pub mod graphics;
pub mod memory;

pub fn initialize() {
    //cpu::interrupts::gdt::init();
    cpu::interrupts::init();
}
